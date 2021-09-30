---
description: Your favourite quotes are just an HTTP request away!
---

# Quotes

The Bender server provides you with the ability to fetch random quotes on demand. It does this through a simple HTTP API and is able to return responses in multiple formats, depending on the client's `Accept` header. By default \(because we're all lazy developers\) it will return content in `application/json` format.

{% api-method method="get" host="https://bender.sierrasoftworks.com" path="/api/v1/quote/:person" %}
{% api-method-summary %}
Get Quote
{% endapi-method-summary %}

{% api-method-description %}
This endpoint allows you to fetch a quote from your favourite character from Futurama. If you don't provide a person's name, a random quote from any character will be returned.
{% endapi-method-description %}

{% api-method-spec %}
{% api-method-request %}
{% api-method-path-parameters %}
{% api-method-parameter name="person" type="string" %}
The name of a character to fetch a quote from
{% endapi-method-parameter %}
{% endapi-method-path-parameters %}

{% api-method-headers %}
{% api-method-parameter name="Accept" type="string" required=false %}
The content format you wish to receive a response in.
{% endapi-method-parameter %}
{% endapi-method-headers %}
{% endapi-method-request %}

{% api-method-response %}
{% api-method-response-example httpCode=200 %}
{% api-method-response-example-description %}
We found a quote for you, you're welcome!
{% endapi-method-response-example-description %}

{% tabs %}
{% tab title="application/json" %}
```javascript
{
    "quote": "Bite my shiny metal ass.",
    "who": "Bender"
}
```
{% endtab %}

{% tab title="text/plain" %}
```
Bite my shiny metal ass. – Bender
```
{% endtab %}

{% tab title="text/html" %}
```markup
<html>
    <head>
        <style>
            body {
                font-family: Sans-serif;
            }

            figure {
                margin: 20px;
            }

            blockquote {
                margin-left: 1em;
            }

            figcaption {
                margin-left: 2em;
                font-size: 0.8em;
                font-weight: bold;
            }

            figcaption::before {
                display: inline;
                content: "–";
                padding-right: 0.5em;
            }
        </style>
    </head>
    <body>
        <figure>
            <blockquote>Bite my shiny metal ass.</blockquote>
            <figcaption>Bender</figcaption>
        </figure>
    </body>
</html>
```
{% endtab %}
{% endtabs %}
{% endapi-method-response-example %}

{% api-method-response-example httpCode=404 %}
{% api-method-response-example-description %}
Could not find a quote by the provided author.
{% endapi-method-response-example-description %}

```javascript
{
    "code": 404,
    "error": "Not Found",
    "message": "We could not find the resource you were looking for, please check your request and try again."
}
```
{% endapi-method-response-example %}
{% endapi-method-response %}
{% endapi-method-spec %}
{% endapi-method %}



