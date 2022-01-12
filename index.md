## fumo-api

fumo-api is a rest API to get pictures and videos of random fumos (plushies of tohou characters), feel free to contribute

# Docs 

## Objects
### fumo 
```json
{
  "_id": number,
  "URL": string,
  "__v": 0
}
```

## Endpoints

### `/random`

return a random fumo object

### `/fumos`
Return the full fumo list 

### `/fumos/<ID>`
Return the fumo with the provided id
