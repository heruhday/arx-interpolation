call %vc19%

set "root=D:\lisp\arx\interpolation"
cd /D %root% && cargo build --release
msbuild.exe .\cpp\interpolation.vcxproj /property:Configuration=Release
