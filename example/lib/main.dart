import 'package:flutter/material.dart';
import 'package:discord_presence/discord_presence.dart';

Future<void> main() async {
  await RustLib.init();

  setDiscordRpc(
      buttons: [const DiscordButton(label: "Test", url: "URL")],
      clientId: "DISCORD_CLIENT_ID",
      details: "Details",
      largeImageKey: "large_image",
      largeTextKey: "",
      smallImageKey: "",
      smallTextKey: "",
      state: "Discord Presence");
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('flutter_rust_bridge quickstart')),
        body: const Center(
          child: Text('Done'),
        ),
      ),
    );
  }
}
