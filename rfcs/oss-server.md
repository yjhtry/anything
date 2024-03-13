# Oss Server

- Feature Name: oss-server
- Start Date: 2024-03-04 18:14:42

## Summary

This RFC proposes the creation of a oss server.

## Motivation

When anything app start, it will start a oss server and in the oss page we can manage oss resources.

## Reference-level explanation

Basic architecture:

![oss-arch](/sreenshot/oss-arch.png)

### Events

```ts
invoke('move_file_to_oss', {})

invoke('get_oss_tree', {})

invoke('get_app_settings', {})

invoke('remove_file_from_oss', {
  path: string,
})
```

### Database

N/A

## Drawbacks

N/A

## Prior art

N/A

## Unresolved questions

## Future possibilities
