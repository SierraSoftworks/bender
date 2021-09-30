---
description: Check the health of your Bender server.
---

# Health

If you're hosting your own Bender instance, you might find it handy to be able to check whether the server is working correctly. The easiest way to do this is by hitting the [health endpoint](health.md#get-health) and checking the response status code.

{% api-method method="get" host="https://bender.sierrasoftworks.com" path="/api/v1/health" %}
{% api-method-summary %}
Get Health
{% endapi-method-summary %}

{% api-method-description %}
This endpoint returns health information for the active Bender server instance.
{% endapi-method-description %}

{% api-method-spec %}
{% api-method-request %}

{% api-method-response %}
{% api-method-response-example httpCode=200 %}
{% api-method-response-example-description %}
The service is healthy
{% endapi-method-response-example-description %}

```javascript
{
    "ok":true
}
```
{% endapi-method-response-example %}

{% api-method-response-example httpCode=500 %}
{% api-method-response-example-description %}
The service is unhealthy.
{% endapi-method-response-example-description %}

```
{
    "code": 500,
    "error": "Internal Server Error",
    "message": "The service is not working correctly right now."
}
```
{% endapi-method-response-example %}

{% api-method-response-example httpCode=503 %}
{% api-method-response-example-description %}
The service is not responding to requests.
{% endapi-method-response-example-description %}

```

```
{% endapi-method-response-example %}
{% endapi-method-response %}
{% endapi-method-spec %}
{% endapi-method %}



