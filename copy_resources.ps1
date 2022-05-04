param(
    [switch]
    $release
)

$dst = "./target/debug"

if $release {
    $dst = "./target/release"
}

Copy-Item "./resources" -Destination $dst -Recurse