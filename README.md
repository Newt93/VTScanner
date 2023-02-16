# VTScanner
A VirusTotal file scanner in Rust // incomplete

Current errors are:

error[E0432]: unresolved import `reqwest::multipart::reader`
 --> src/main.rs:3:5
  |
3 | use reqwest::multipart::reader;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ no `reader` in `async_impl::multipart`

error[E0433]: failed to resolve: could not find `reader` in `multipart`
  --> src/main.rs:20:85
   |
20 |         .multipart(reqwest::multipart::Form::new().part("file", reqwest::multipart::reader::CursorReader::new(cursor)))
   |                                                                                     ^^^^^^ could not find `reader` in `multipart`

error[E0599]: no method named `unwrap` found for reference `&str` in the current scope
  --> src/main.rs:28:10
   |
28 |         .unwrap()
   |          ^^^^^^ method not found in `&str`

