param(
    [switch]
    $release
)

$dst = "./target/debug"

if ($release) {
    $dst = "./target/release"
}

if( Test-Path $dst/resources ) {
    Remove-Item $dst/resources -Recurse
}
Copy-Item "./resources" -Destination $dst -Recurse