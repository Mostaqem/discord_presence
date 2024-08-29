# discord_presence
Discord RPC for dart/flutter written in Rust (Work In Progress)

I was using [discord_rpc](https://pub.dev/packages/discord_rpc) and it was working great but it was missing some features like it doesn't work on MacOs and I needed it for [Mostaqem](https://github.com/Mostaqem/mostaqem_desktop), so I decided to create one using my Rust skills + new feature (Custom Buttons and no more than 2 buttons) 



## Getting Started
For integrating Discord Rich Presence into your application or game, you must create an application at [Discord Developer Portal](https://discord.com/developers/applications).

Set or change the discord presence
```dart
  setDiscordRpc(
      buttons: [const DiscordButton(label: "Test", url: "URL")],
      clientId: "DISCORD_CLIENT_ID",
      details: "Details",
      largeImageKey: "large_image",
      largeTextKey: "large_text",
      smallImageKey: "small_image",
      smallTextKey: "small_text",
      state: "Discord Presence");
```

## License
[MIT](LICENSE)