# fumo rest API

[![Codacy Badge](https://api.codacy.com/project/badge/Grade/a7dd96acbca0480788311a77d47b3208)](https://app.codacy.com/gh/Nosesisaid/fumo-API?utm_source=github.com&utm_medium=referral&utm_content=Nosesisaid/fumo-API&utm_campaign=Badge_Grade_Settings)

## Issues
If you find a repeated fumo please create an issue
## Objects

### Fumo

```json
{
  "_id": "fumo id",
  "URL": "fumo image url",
  "__v": 0
}
```

## Paths

### <code>/random</code>

Return a random fumo object.

### <code> /fumos</code>

Fumo list.

#### <code>/</code>

Return the full fumo list.

#### <code>/id</code>

Return the fumo object with the provided ID.
