import Foundation

let path = CommandLine.arguments[1]

print("opening \(path)");

let fileHandle = FileHandle(forWritingAtPath: path)
while true {
    print("writing");
    fileHandle?.write("A".data(using: .utf8)!)
    try fileHandle?.synchronize()
    sleep(1)
}
