#[derive(Debug, PartialEq, Clone)] 
enum Genre {
    Fiction,
    NonFiction,
    Science,
    History,
}

#[derive(Debug, PartialEq, Clone)]  // Add Clone here
struct Book {
    title: String,
    author: String,
    genre: Genre,
    year: i32,
}


impl Book {
    // Constructor to create a new book
    fn new(title: &str, author: &str, genre: Genre, year: i32) -> Self {
        Self {
            title: title.to_string(),
            author: author.to_string(),
            genre,
            year,
        }
    }

    // Method to change the book's year of publication
    fn change_year(&mut self, new_year: i32) {
        self.year = new_year;
    }

    // Method to display book details
    fn display(&self) -> String {
        format!(
            "Title: {}, Author: {}, Genre: {:?}, Year: {}",
            self.title, self.author, self.genre, self.year
        )
    }
}

// Function to sort books by year (descending)
fn sort_books_by_year(mut books: Vec<Book>) -> Vec<Book> {
    books.sort_by(|a, b| b.year.cmp(&a.year));
    books
}

// Function to filter books by genre
fn filter_books_by_genre(books: Vec<Book>, genre: Genre) -> Vec<Book> {
    books.into_iter().filter(|b| b.genre == genre).collect()
}

// Function to find books published after a certain year
fn books_after_year(books: Vec<Book>, year: i32) -> Vec<Book> {
    books.into_iter().filter(|b| b.year > year).collect()
}

// Custom error type for handling invalid book data
#[derive(Debug)]
enum BookError {
    InvalidYear,
    InvalidGenre,
}

// Function to create a book with error handling
fn create_book_with_error_handling(
    title: &str,
    author: &str,
    genre: Genre,
    year: i32,
) -> Result<Book, BookError> {
    if year < 0 {
        Err(BookError::InvalidYear)
    } else if genre == Genre::Fiction && title.is_empty() {
        Err(BookError::InvalidGenre)
    } else {
        Ok(Book::new(title, author, genre, year))
    }
}
fn main() {
    // Create books using the create_book_with_error_handling function
    let book1 = create_book_with_error_handling("The Rust Programming Language", "Steve Klabnik", Genre::NonFiction, 2018);
    let book2 = create_book_with_error_handling("Learning Rust", "Jane Doe", Genre::Fiction, 2020);
    let book3 = create_book_with_error_handling("Rust in Action", "John Smith", Genre::Science, 2019);
    let book4 = create_book_with_error_handling("The History of Rust", "Alex Brown", Genre::History, 2022);

    // Collect all books into a vector, filtering out any errors
    let books = vec![
        book1.ok(),
        book2.ok(),
        book3.ok(),
        book4.ok(),
    ]
    .into_iter()
    .filter_map(|x| x)
    .collect::<Vec<Book>>();

    // Display the details of the books
    for book in &books {
        println!("{}", book.display());
    }

    // Sort books by year (descending)
    let sorted_books = sort_books_by_year(books.clone());
    println!("\nSorted Books by Year:");
    for book in sorted_books {
        println!("{}", book.display());
    }

    // Filter books by genre (e.g., Fiction)
    let fiction_books = filter_books_by_genre(books.clone(), Genre::Fiction);
    println!("\nFiction Books:");
    for book in fiction_books {
        println!("{}", book.display());
    }

    // Find books published after 2019
    let books_after_2019 = books_after_year(books.clone(), 2019);
    println!("\nBooks Published After 2019:");
    for book in books_after_2019 {
        println!("{}", book.display());
    }
}

// // Unit Tests
// #[cfg(test)]
// mod tests {
//     use super::*;

//     // Test for creating a valid book
//     #[test]
//     fn test_create_book_valid() {
//         let book = Book::new("The Rust Programming Language", "Steve Klabnik", Genre::NonFiction, 2018);
//         assert_eq!(book.title, "The Rust Programming Language");
//         assert_eq!(book.year, 2018);
//     }

//     // Test for changing book year
//     #[test]
//     fn test_change_book_year() {
//         let mut book = Book::new("Learn Rust", "John Doe", Genre::Science, 2021);
//         book.change_year(2023);
//         assert_eq!(book.year, 2023);
//     }

//     // Test for sorting books by year
//     #[test]
//     fn test_sort_books_by_year() {
//         let books = vec![
//             Book::new("Rust for Beginners", "Alice", Genre::Fiction, 2020),
//             Book::new("Advanced Rust", "Bob", Genre::NonFiction, 2021),
//             Book::new("Rust in Action", "Charlie", Genre::Science, 2019),
//         ];

//         let sorted_books = sort_books_by_year(books);
//         assert_eq!(sorted_books[0].year, 2021);
//         assert_eq!(sorted_books[1].year, 2020);
//         assert_eq!(sorted_books[2].year, 2019);
//     }

//     // Test for filtering books by genre
//     #[test]
//     fn test_filter_books_by_genre() {
//         let books = vec![
//             Book::new("The Rust Book", "Steve Klabnik", Genre::NonFiction, 2020),
//             Book::new("Rust for Beginners", "Alice", Genre::Fiction, 2021),
//             Book::new("Rust in Action", "Charlie", Genre::Science, 2021),
//         ];

//         let filtered_books = filter_books_by_genre(books, Genre::Fiction);
//         assert_eq!(filtered_books.len(), 1);
//         assert_eq!(filtered_books[0].title, "Rust for Beginners");
//     }

//     // Test for filtering books after a specific year
//     #[test]
//     fn test_books_after_year() {
//         let books = vec![
//             Book::new("The Rust Book", "Steve Klabnik", Genre::NonFiction, 2020),
//             Book::new("Rust for Beginners", "Alice", Genre::Fiction, 2021),
//             Book::new("Rust in Action", "Charlie", Genre::Science, 2019),
//         ];

//         let filtered_books = books_after_year(books, 2020);
//         assert_eq!(filtered_books.len(), 1);
//         assert_eq!(filtered_books[0].title, "Rust for Beginners");
//     }

//     // Test for error handling when creating a book with invalid year
//     #[test]
//     fn test_create_book_invalid_year() {
//         let result = create_book_with_error_handling("Rust Basics", "John Doe", Genre::Fiction, -2021);
//         assert_eq!(result, Err(BookError::InvalidYear));
//     }

//     // Test for error handling when creating a book with missing title for Fiction genre
//     #[test]
//     fn test_create_book_invalid_genre() {
//         let result = create_book_with_error_handling("", "John Doe", Genre::Fiction, 2021);
//         assert_eq!(result, Err(BookError::InvalidGenre));
//     }
// }
