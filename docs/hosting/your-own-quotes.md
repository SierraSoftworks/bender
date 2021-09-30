---
description: 'If you don''t like Futurama, you can provide your own list of quotes instead.'
---

# Your own Quotes

If you're the type of vain individual who wants to force their own nuggets of "wisdom" on the unsuspecting masses, you can do that by providing a custom `quotes.json` file for your server. Oh sure, you can use it to serve quotes from other shows or famous people, but we all really know what you're up to.

The first step is putting together a quotes file. This is just as painful as it should be to discourage the creation of new quotes, but if you really want to do it, the file looks like this:

```javascript
[
    { "quote": "Not even once...", "who": "You" },
    { "quote": "Seriously, not even once...", "who": "You" }
]
```

When you run the server container make sure that you mount your custom quotes file at `/app/quotes.json`.

```text
$ docker run -p 8000:8000 -v ./quotes.json:/app/quotes.json \
    ghcr.io/sierrasoftworks/bender/bender
```

