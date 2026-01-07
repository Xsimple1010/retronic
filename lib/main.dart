import 'dart:io';

import 'package:flutter/material.dart';
import 'package:retronic/components/base/retro_elevated_button.dart';
import 'package:retronic/pages/openning.dart';
import 'package:rinf/rinf.dart';

import 'src/bindings/bindings.dart';

Future<void> main() async {
  await initializeRust(assignRustSignal);
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(colorScheme: .fromSeed(seedColor: Colors.deepPurple)),
      home: OpeningPage(),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key, required this.title});

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  bool closeBtVisibility(GameStateChange state) {
    switch (state) {
      case GameStateChange.running:
        return true;
      case GameStateChange.closed:
        return false;
      case GameStateChange.paused:
        return true;
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
        title: Text(widget.title),
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: .center,
          children: [
            const Text('You have pushed the button this many times:'),
            RetroElevatedButton(
              onPressed: () {
                LoadGame(
                  romPath: "/home/aderval/Downloads/RetroArch_cores/ff.smc",
                  corePath:
                      "/home/aderval/Downloads/RetroArch_cores/RetroArch-Linux-x86_64/RetroArch-Linux-x86_64.AppImage.home/.config/retroarch/cores/snes9x_libretro.so",
                  baseRetroPath:
                      "/home/aderval/Downloads/RetroArch_cores/retronic",
                ).sendSignalToRust();
              },
              child: Text("Jogar"),
            ),

            StreamBuilder(
              stream: GameStateChangeSignal.rustSignalStream,
              builder: (context, snapshot) => Visibility(
                visible: snapshot.data?.message.state != null
                    ? closeBtVisibility(snapshot.data!.message.state)
                    : false,
                child: RetroElevatedButton(
                  onPressed: () {
                    CloseGame().sendSignalToRust();
                  },
                  child: Text("fechar"),
                ),
              ),
            ),

            StreamBuilder(
              stream: SaveStateInfoSignal.rustSignalStream,
              builder: (context, snapshot) => Visibility(
                visible: snapshot.data?.message.saveImgPreview != null,
                replacement: Text("sem imagem"),
                child: Image.file(
                  File(snapshot.data?.message.saveImgPreview ?? ""),
                ),
              ),
            ),

            StreamBuilder(
              stream: DeviceConnectedSignal.rustSignalStream,
              builder: (context, snapshot) =>
                  Text("Controle name: ${snapshot.data?.message.name}"),
            ),
          ],
        ),
      ),
    );
  }
}
