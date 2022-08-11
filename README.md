# Fulltext Search Rust

This rust application is essentially a better version of [github.com/obrhubr/fulltext-search-cpp](https://github.com/obrhubr/fulltext-search-cpp). 

The key value store used for the inverted index is `rocksdb`. To serialize and deserialize the values, `bincode` and `serde` are used. To store the actual text, `sqlite` is used. The library used to create the web interface is `actix-web` with `serde_json` being the serializer and deserializer.

### Architecture 

The search engine relies on an inverted index to make the books searchable. For each word that exists in the books, a key is created in the KV store (RocksDB) and each occurence of the word is stored as a pair of (book_id, position_in_book). If a user searches for "Book" for example, the engine will first get the positions of each occurence, then fetch the sourrounding text from the book and return the results after ranking them. If the user searches for multiple words, ex: "Comic Book", the engine will fetch the occurences for both word, and if there are occurences of both words in close proximity in the same book, it will rank that higher.

### Usage

There are 5 routes. All of them accept only json: 
 - `/add` : Adding a book to make it searchable
 - `/edit` : Edit a book
 - `/remove` : Remove a book
 - `/search` : Search the text of books


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

#### `/search/all`
To use the `/search/all` route, send:
```
{
    "query": The text you want to search for,
    "peri_text_length": Length of the text around the search result to be sent back
}
```
It will send back data resembling this:
```
{
    "results": [
        {
            "id": "1",
            "peri_text": "test ",
            "word": 0
        }
    ]
}
```

#### `/search/one`
To use the `/search/one` route, send:
```
{
    "id": The id of the book you want to search in,
    "query": The text you want to search for,
    "peri_text_length": Length of the text around the search result to be sent back
}
```
It will send back data resembling this:
```
{
    "results": [
        {
            "id": "1",
            "peri_text": "test ",
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
