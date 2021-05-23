# 库文件夹名
$CSFML = "CSFML-2.5-windows-64-bit"

# 主站与镜像站
$FASTGIT = "download.fastgit.org"
$GITHUB = "github.com"

# 获取访问速度最快的主机
function Get-Fastest-Host {
    param(
        [string[]]$HOSTS
    )
    $restime = 9999
    foreach($h in $HOSTS) {
        $_restime = (Test-Connection $h -Count 1).ResponseTime
        if($_restime -le $restime) {
            $fastesthost = $h
        }
        $restime = $_restime
    }
    $fastesthost
}

# 下载CSFML-2.5-windows-64-bit静态与动态链接库
# 解压并去除多余文件
if(!(Test-Path $CSFML)) {
    $CSFML_GITHUB_RELEASE = `
    "https://$(Get-Fastest-Host -HOSTS $FASTGIT, $GITHUB)/SFML/CSFML/releases/download/2.5/$CSFML.zip"
    
    Invoke-WebRequest `
    -Uri $CSFML_GITHUB_RELEASE `
    -OutFile "$CSFML.zip"

    Expand-Archive `
    -Path "$CSFML.zip" `
    -DestinationPath $CSFML `
    -Force

    Remove-Item `
    -Path "$CSFML.zip",".\$CSFML\include",".\$CSFML\license.txt",".\$CSFML\readme.txt" `
    -Recurse `
    -Force
}

# 设置环境变量CSFML_HOME
# Cargo 构建
if(Test-Path "build.rs") {
    $env:CSFML_HOME = (Get-Item $CSFML).FullName
    Cargo build
}