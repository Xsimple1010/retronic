import 'package:flutter/material.dart';
import 'package:retronic/components/base/retro_elevated_button.dart';
import 'package:retronic/components/base/retro_icon_button.dart';

class OpeningPage extends StatefulWidget {
  const OpeningPage({super.key});

  @override
  State<OpeningPage> createState() => _OpeningPageState();
}

class _OpeningPageState extends State<OpeningPage> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Center(
        child: Column(
          children: [
            Text("Vamos começa!"),
            RetroIconButton(
              onPressed: () {},
              icon: const Icon(Icons.arrow_forward, size: 72),
            ),
            RetroElevatedButton(child: Text(""), onPressed: () {}),
          ],
        ),
      ),
    );
  }
}
