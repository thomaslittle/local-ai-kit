"""Verify distributable artifacts from clean consumers, without inference or publication."""
import argparse,json,os,pathlib,shutil,subprocess,sys,tempfile,zipfile
from release_support import verify_manifest
ROOT=pathlib.Path(__file__).resolve().parents[1]

def run(command,cwd,env=None):
    exe=shutil.which(str(command[0]))
    if not exe:raise RuntimeError('Missing consumer tool: '+str(command[0]))
    args=[exe,*map(str,command[1:])]
    if os.name=='nt' and exe.lower().endswith(('.cmd','.bat')):args=[os.environ.get('COMSPEC','cmd.exe'),'/d','/c',*args]
    result=subprocess.run(args,cwd=cwd,env=env,capture_output=True,text=True,encoding='utf-8',errors='replace',timeout=240)
    if result.returncode:raise RuntimeError('Consumer check failed: '+str(command[0])+'\n'+result.stdout[-6000:]+result.stderr[-6000:])
    return result.stdout

def main():
    parser=argparse.ArgumentParser();parser.add_argument('--bundle',type=pathlib.Path);args=parser.parse_args()
    if args.bundle:bundle=args.bundle.resolve()
    else:
        pointer=ROOT/'dist/release/latest.json'
        if not pointer.is_file():raise RuntimeError('Build a release bundle first: python scripts/build_release.py')
        bundle=(ROOT/json.loads(pointer.read_text(encoding='utf-8'))['directory']).resolve()
    manifest=json.loads((bundle/'manifest.json').read_text(encoding='utf-8'));verify_manifest(bundle,manifest)
    suffix='.exe' if os.name=='nt' else '';binary=bundle/'bin'/('local-ai-kit'+suffix)
    with tempfile.TemporaryDirectory(prefix='lai-clean-consumer-') as directory:
        base=pathlib.Path(directory);home=base/'empty-home';home.mkdir()
        # This executable is copied out of the workspace's target directory.
        assert 'local-ai-kit' in run([binary,'--version'],base)
        snapshot=json.loads(run([binary,'scan','--json','--home',home],base));assert snapshot['mode']=='live' and not snapshot['connections'] and not snapshot['models']
        consumer=base/'node';consumer.mkdir()
        package_paths=sorted((bundle/'packages').glob('*.tgz'))
        if len(package_paths)!=4:raise RuntimeError('Expected four JavaScript artifacts')
        (consumer/'package.json').write_text(json.dumps({'private':True,'type':'module'}),encoding='utf-8')
        dependencies=json.loads((ROOT/'package.json').read_text(encoding='utf-8'))['devDependencies']
        run(['npm','install','--offline','--ignore-scripts','--no-audit','--no-fund',*package_paths,'react@'+dependencies['react'],'react-dom@'+dependencies['react-dom']],consumer)
        script="import assert from 'node:assert/strict';\nimport {createLocalAI,createSnapshotStore} from '@local-ai-kit/sdk';\nimport {openBundledTransport} from '@local-ai-kit/sdk/node';\nimport {createCatalog} from '@local-ai-kit/selector-core';\nimport {LocalAIProvider} from '@local-ai-kit/react';\nimport {ProviderModelSelector} from '@local-ai-kit/ui-react';\nimport React from 'react';\nimport {renderToString} from 'react-dom/server';\nconst t=await openBundledTransport({bundleDirectory:process.argv[2],args:['serve','--stdio','--home',process.argv[3],'--allow-passive-discovery']});\nconst ai=await createLocalAI({transport:t});\ntry {\n assert.equal((await ai.snapshot.get()).connections.length,0);\n await ai.discovery.refresh({level:'passive'});\n const snapshot=await ai.snapshot.get();assert.equal(snapshot.mode,'live');assert.equal(snapshot.connections.length,0);\n assert.equal(createCatalog(snapshot).models.size,0);\n const store=createSnapshotStore(ai);assert.equal(store.getSnapshot().snapshot,null);assert.equal(store.getSnapshot().loading,false);\n const html=renderToString(React.createElement(LocalAIProvider,{store},React.createElement(ProviderModelSelector,{snapshot,value:null,onValueChange:()=>{},variant:'rail'})));\n assert.ok(html.length>0);store.dispose();\n console.log('PASS packaged JS SDK/native scan and React server rendering');\n} finally {await ai.close();}\n"
        (consumer/'consumer.mjs').write_text(script,encoding='utf-8')
        js=run(['node',consumer/'consumer.mjs',bundle,home],consumer)
        wheel=list((bundle/'packages').glob('*.whl'))
        if len(wheel)!=1:raise RuntimeError('Expected one Python wheel')
        with zipfile.ZipFile(wheel[0]) as archive:
            names=archive.namelist()
            for required in ('local_ai_kit/client.pyi','local_ai_kit/generated.py','local_ai_kit/py.typed'):
                if required not in names:raise RuntimeError('Wheel lacks '+required)
        destination=base/'python';destination.mkdir()
        run([sys.executable,'-m','pip','install','--no-index','--no-deps','--target',destination,wheel[0]],base)
        program="import pathlib,sys\nsys.path.insert(0,sys.argv[1])\nimport local_ai_kit\nfrom local_ai_kit import Client, resolve_bundle_binary\nfrom local_ai_kit.generated import Snapshot\nassert pathlib.Path(local_ai_kit.__file__).resolve().is_relative_to(pathlib.Path(sys.argv[1]).resolve())\nwith Client.from_stdio(executable=resolve_bundle_binary(sys.argv[2]),args=['serve','--stdio','--home',sys.argv[3],'--allow-passive-discovery']) as ai:\n    assert ai.snapshot.get()['connections']==[]\n    ai.discovery.refresh({'level':'passive'})\n    assert ai.snapshot.get()['mode']=='live'\n    assert ai.models.list()==[]\nprint('PASS installed Python wheel with actual native helper')\n"
        script_path=base/'consumer.py';script_path.write_text(program,encoding='utf-8')
        py=run([sys.executable,'-I',script_path,destination,bundle,home],base)
    report={'sourceCommit':manifest['sourceCommit'],'workingTreeDirty':manifest['workingTreeDirty'],'platform':manifest['platform'],'checks':{'manifest':True,'standaloneNative':True,'npmTarballs':True,'pythonWheel':True,'reactSSR':True},'inferenceRequests':0,'limitations':['No remote provider inference or OS signing verification','Registry publication was not attempted']}
    out=ROOT/'.verification/package-smoke.json';out.parent.mkdir(exist_ok=True);out.write_text(json.dumps(report,indent=2)+'\n',encoding='utf-8')
    print(js.strip());print(py.strip());print(json.dumps(report,indent=2))
if __name__=='__main__':
    try:main()
    except (AssertionError,ValueError,RuntimeError,OSError,subprocess.SubprocessError)as error:print(str(error),file=sys.stderr);raise SystemExit(1)
