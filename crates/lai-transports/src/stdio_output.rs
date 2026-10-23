//! One bounded output worker independent of provider blocking-pool work.
use std::{
    io::{self, Write},
    pin::Pin,
    sync::mpsc::{self, SyncSender},
    task::{Context, Poll},
};
use tokio::{io::AsyncWrite, sync::oneshot};
enum Operation {
    Write(Vec<u8>),
    Flush,
}
#[derive(Clone, Copy)]
enum Completion {
    Write(usize),
    Flush,
}
struct Request {
    operation: Operation,
    done: oneshot::Sender<io::Result<()>>,
}
/// A dedicated stdout worker prevents native-process reaping or inventory work
/// from starving protocol output in the runtime's bounded blocking pool.
/// Only one request is in flight; no payload is logged.
pub fn stdout() -> io::Result<Output> {
    Output::new(io::stdout())
}
pub struct Output {
    sender: Option<SyncSender<Request>>,
    pending: Option<(Completion, oneshot::Receiver<io::Result<()>>)>,
}
impl Output {
    fn new(mut writer: impl Write + Send + 'static) -> io::Result<Self> {
        let (sender, requests) = mpsc::sync_channel::<Request>(1);
        std::thread::Builder::new()
            .name("lai-stdio-output".into())
            .spawn(move || {
                while let Ok(request) = requests.recv() {
                    let result = match request.operation {
                        Operation::Write(bytes) => writer.write_all(&bytes),
                        Operation::Flush => writer.flush(),
                    };
                    let failed = result.is_err();
                    if request.done.send(result).is_err() || failed {
                        break;
                    }
                }
            })?;
        Ok(Self {
            sender: Some(sender),
            pending: None,
        })
    }
    fn submit(&mut self, operation: Operation, completion: Completion) -> io::Result<()> {
        let (done, result) = oneshot::channel();
        self.sender
            .as_ref()
            .ok_or_else(closed)?
            .try_send(Request { operation, done })
            .map_err(|_| closed())?;
        self.pending = Some((completion, result));
        Ok(())
    }
    fn complete(&mut self, cx: &mut Context<'_>) -> Poll<io::Result<Completion>> {
        let Some((kind, result)) = self.pending.as_mut() else {
            return Poll::Ready(Ok(Completion::Flush));
        };
        let result = match Pin::new(result).poll(cx) {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(result) => result,
        };
        let kind = *kind;
        self.pending = None;
        Poll::Ready(result.map_err(|_| closed()).and_then(|r| r).map(|()| kind))
    }
}
fn closed() -> io::Error {
    io::Error::new(io::ErrorKind::BrokenPipe, "Protocol output worker closed")
}
impl AsyncWrite for Output {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bytes: &[u8],
    ) -> Poll<io::Result<usize>> {
        if self.pending.is_some() {
            match std::task::ready!(self.complete(cx))? {
                Completion::Write(count) => return Poll::Ready(Ok(count)),
                Completion::Flush => {}
            }
        }
        if bytes.is_empty() {
            return Poll::Ready(Ok(0));
        }
        let count = bytes.len().min(65536);
        self.submit(
            Operation::Write(bytes[..count].to_vec()),
            Completion::Write(count),
        )?;
        match std::task::ready!(self.complete(cx))? {
            Completion::Write(count) => Poll::Ready(Ok(count)),
            Completion::Flush => unreachable!(),
        }
    }
    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        if self.pending.is_some() {
            match std::task::ready!(self.complete(cx))? {
                Completion::Flush => return Poll::Ready(Ok(())),
                Completion::Write(_) => {}
            }
        }
        self.submit(Operation::Flush, Completion::Flush)?;
        std::task::ready!(self.complete(cx))?;
        Poll::Ready(Ok(()))
    }
    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        std::task::ready!(self.as_mut().poll_flush(cx))?;
        self.sender = None;
        Poll::Ready(Ok(()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    };
    use tokio::io::AsyncWriteExt;
    struct Sink(Arc<Mutex<Vec<u8>>>);
    impl Write for Sink {
        fn write(&mut self, b: &[u8]) -> io::Result<usize> {
            self.0.lock().unwrap().extend_from_slice(b);
            Ok(b.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    #[tokio::test]
    async fn chunked_output_and_flush_preserve_exact_bytes() {
        let bytes = Arc::new(Mutex::new(Vec::new()));
        let mut output = Output::new(Sink(bytes.clone())).unwrap();
        let expected = vec![42; 262144];
        output.write_all(&expected).await.unwrap();
        output.flush().await.unwrap();
        output.shutdown().await.unwrap();
        assert_eq!(*bytes.lock().unwrap(), expected);
    }
    struct Gate {
        entered: Option<std::sync::mpsc::Sender<()>>,
        release: std::sync::mpsc::Receiver<()>,
        dropped: Arc<AtomicBool>,
    }
    impl Write for Gate {
        fn write(&mut self, b: &[u8]) -> io::Result<usize> {
            self.entered.take().unwrap().send(()).unwrap();
            self.release.recv().unwrap();
            Ok(b.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            panic!("cancelled output must not issue another operation")
        }
    }
    impl Drop for Gate {
        fn drop(&mut self) {
            self.dropped.store(true, Ordering::Release);
        }
    }
    #[tokio::test]
    async fn dropping_cancelled_output_stops_after_owned_pending_write() {
        let (entered, ready) = std::sync::mpsc::channel();
        let (release, gate) = std::sync::mpsc::channel();
        let dropped = Arc::new(AtomicBool::new(false));
        let mut output = Output::new(Gate {
            entered: Some(entered),
            release: gate,
            dropped: dropped.clone(),
        })
        .unwrap();
        assert!(
            tokio::time::timeout(
                std::time::Duration::from_millis(20),
                output.write_all(&[1; 65536])
            )
            .await
            .is_err()
        );
        ready
            .recv_timeout(std::time::Duration::from_secs(1))
            .unwrap();
        drop(output);
        release.send(()).unwrap();
        tokio::time::timeout(std::time::Duration::from_secs(1), async {
            while !dropped.load(Ordering::Acquire) {
                tokio::task::yield_now().await;
            }
        })
        .await
        .unwrap();
    }
}
