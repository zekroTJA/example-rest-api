# Example REST API

An open source CRUD REST API to build demos on, test stuff against or just play around with.

## Motivation

For other projects, I was using [restful-api.dev](https://restful-api.dev/) to build demos or tests on. But recently, this page proved to be somewhat inconsistent and unreliable, so I wanted to build something hosted on [Vercel](https://vercel.com) using [Upstash](https://upstash.com/) as Redis Store, so that it should be a pretty reliable alternative.

## API Documentation

The live API root endpoint is as following.
```
https://example-rest-api.vercel.app
```

The API accepts request payloads as `application/json` and responses are encoded in `application/json`.

> **Warning**  
> Created objects are **not persistent nor protected**. Objects are usually automatically removed 24 hours after creation or modification. Objects and collection are not proof against modification by unauthorized individuals.

### Objects

#### `Collection`

```ts
type CollectionRequest = {
    name: string;
};

type CollectionResponse = {
    id: string;
    created_at: string; // Format: RFC3339
    name: string;
};
```

#### `Object`

```ts
type ObjectRequest = {
    name: string;
    data: { [key: string]: any };
};

type ObjectResponse = {
    id: string;
    created_at: string; // Format: RFC3339
    name: string;
    data: { [key: string]: any };
};
```

### Endpoints

#### Create Collection

Create a new collection.

```
POST /api/collections
```

##### Request Payload

```ts
CollectionRequest
```

*Example:*
```json
{
    "name": "my games"
}
```

##### Response Payload

```ts
CollectionResponse
```

*Example:*
```json
{
	"id": "chf13ur2iejc717547r0",
	"created_at": "2023-05-12T10:16:27.319442753+00:00",
	"name": "my games"
}
```

#### Get Collection

Retrieve information about a collection by ID.

```
GET /api/collections/:collectionid
```

##### Response Payload

```ts
CollectionResponse
```

*Example:*
```json
{
	"id": "chf13ur2iejc717547r0",
	"created_at": "2023-05-12T10:16:27.319442753+00:00",
	"name": "my games"
}
```

#### Delete Collection

Delete a collection by ID.

```
DELETE /api/collections/:collectionid
```

#### Create Object

Create a new object in a collection.

```
POST /api/collections/:collectionid/objects
```

##### Request Payload

```ts
ObjectRequest
```

*Example:*
```json
{
    "name": "Cyberpunk 2077",
    "data": {
        "publisher": "CD PROJECT RED",
        "developer": "CD PROJECT RED",
        "released": "2020-12-10T00:00:00Z",
        "tags": ["Cyberpunk", "Open World", "RPG", "Sci-fi"],
        "age_rating": "18"
    }
}
```

##### Response Payload

```ts
ObjectResponse
```

*Example:*
```json
{
	"id": "chf15eacaqvs715h7ae0",
	"created_at": "2023-05-12T10:19:37.971387217+00:00",
	"name": "Cyberpunk 2077",
	"data": {
		"tags": [
			"Cyberpunk",
			"Open World",
			"RPG",
			"Sci-fi"
		],
		"age_rating": "18",
		"publisher": "CD PROJECT RED",
		"developer": "CD PROJECT RED",
		"released": "2020-12-10T00:00:00Z"
	}
}
```

#### Get Object

Retrieve an object from a collection by ID.

```
GET /api/collections/:collectionid/objects/:objectid
```

##### Response Payload

```ts
ObjectResponse
```

*Example:*
```json
{
	"id": "chf15eacaqvs715h7ae0",
	"created_at": "2023-05-12T10:19:37.971387217+00:00",
	"name": "Cyberpunk 2077",
	"data": {
		"tags": [
			"Cyberpunk",
			"Open World",
			"RPG",
			"Sci-fi"
		],
		"age_rating": "18",
		"publisher": "CD PROJECT RED",
		"developer": "CD PROJECT RED",
		"released": "2020-12-10T00:00:00Z"
	}
}
```

#### Delete Object

Delete an object in a collection by ID.

```
DELETE /api/collections/:collectionid/objects/:objectid
```