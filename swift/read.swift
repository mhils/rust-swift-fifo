import Foundation

let path = CommandLine.arguments[1]

print("opening \(path)");

let fileHandle = FileHandle(forReadingAtPath: path)
while true {
    print("reading")
    let data = try? fileHandle?.read(upToCount: 1)
    print("data: \(String(describing: data))")
    sleep(1)
}
