import 'package:flutter/material.dart';
import 'package:discord_presence/discord_presence.dart';

void main(){
  setDiscordRpc(
      buttons: [const DiscordButton(label: "Test", url: "URL")],
      clientId: "DISCORD_CLIENT_ID",
      details: "Details",
      largeImageKey: "large_image",
      largeTextKey: "large_text_key",
      smallImageKey: "small_image_key",
      smallTextKey: "small_text_key",
      state: "Discord Presence");

  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('quickstart')),
        body: const Center(
          child: Text('Done'),
        ),
      ),
    );
  }
}
