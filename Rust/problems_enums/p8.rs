// 10. Enum with Recursion
// Problem: Create an enum FileSystem with variants:
// File { name: String, size: u32 }
// Directory { name: String, files: Vec<FileSystem> }
// Write a function get_size that calculates the total size of a directory, including the sizes of all files inside. 
//If a Directory contains other Directory variants, sum the sizes recursively.

enum FileSystem{
    File { name: String, size: u32 },
    Directory { name: String, files: Vec<FileSystem> },
}