fn main() {
    tcrec::run();
}

// TODO
// Look at content of CSV folder
// Process all valid CSV files
// The name of the CSV file is the name of the list
// Make it into slugs by applying lower case and dashes, like "Art Songs" -> "art-songs"
// Read file content into a vector of structs
// Add composer slugs by doing lower case, stripping excessive symbols and replaceing spaces with dashes like "de Silva" -> "de-silva"
// Have a function for building composer top list
// Have a function for filtering works of a single composer
