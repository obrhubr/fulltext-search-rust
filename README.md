# Fulltext Search Rust

This rust application is essentially a better version of [github.com/obrhubr/fulltext-search-cpp](https://github.com/obrhubr/fulltext-search-cpp). 

The key value store used for the inverted index is `rocksdb`. To serialize and deserialize the values, `bincode` and `serde` are used. To store the actual text, `sqlite` is used. The library used to create the web interface is `actix-web` with `serde_json` being the serializer and deserializer.

### Getting Started

To get started, install `librocksdb-dev`, `llvm` and `clang` using your package manager.

### Usage

There are 5 routes. All of them accept only json: 
 - `/add` : Adding a book to make it searchable
 - `/edit` : Edit a book
 - `/remove` : Remove a book
 - `/search/one` : Search the text of a single book
 - `/search/all` : Seach the text of all books


#### `/add`
To use the `/add` route, send:
```
{
    "id": The Id of the book in your main database,
    "name": The name of the book,
    "text": The text in the book
}
```

#### `/edit`
To use the `/edit` route, send:
```
{
    "id": The Id of the book in your main database
    Only send the fields which you want to update(you can send nothing)
    "name": The name of the book,
    "text": The text in the book
}
```

#### `/remove`
To use the `/remove` route, send:
```
{
    "id": The Id of the book in your main database
}
```

#### `/search/one`
To use the `/search/one` route, send:
```
{
    "id": The Id of the book in your main database,
    "searcqueryhText": The text you want to search for,
    "stopAfterOne": (Boolean) If you want to stop after finding the first result
}
```
It will send back data resembling this:
```
{
    "results": [
        {
            "id": "1",
            "periText": "test ",
            "word": 0
        }
    ]
}
```

#### `/search/all`
To use the `/search/all` route, send:
```
{
    "searchText": The text you want to search for,
    "stopAfterOne": (Boolean) If you want to stop after finding the first result
}
```
It will send back data resembling this:
```
{
    "results": [
        {
            "id": "1",
            "periText": "test ",
            "word": 0
        }
    ]
}
```

#### Errors
If an error occurs, the response will look like this:
```
{
    "response": Error in text form
}
```