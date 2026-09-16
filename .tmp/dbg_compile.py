import os, subprocess, sys, tempfile
sys.path.insert(0, "host/zap-bootstrap")
import c_backend as cb

src_path = sys.argv[1] if len(sys.argv) > 1 else "target/c-backend-ds/ds_smoke.zp"
prefix = sys.argv[2] if len(sys.argv) > 2 else "target/c-backend-ds/manual"
with open(src_path, encoding="utf-8-sig") as fh:
    source = fh.read()
cb.emit_c(cb.compile_program(source), prefix + ".c")

cc = cb.find_c_compiler()
vc = cb._find_vcvars(cc)
bat = os.path.join(tempfile.gettempdir(), "zap_dbg_build.bat")
with open(bat, "w", newline="\r\n") as fh:
    fh.write("@echo off\n")
    if vc:
        fh.write('call "%s" >nul\n' % vc)
    fh.write('"%s" /nologo /O2 /Fe:%s.exe %s.c\n' % (cc, prefix, prefix))
r = subprocess.run(["cmd", "/c", bat], capture_output=True, text=True)
print("rc=", r.returncode)
print("STDOUT:")
print(r.stdout[:6000])
print("STDERR:")
print(r.stderr[:6000])