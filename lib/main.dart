import 'package:flutter/material.dart';
import 'package:retronic/pages/download_tinic_ipc.dart';
import 'package:retronic/pages/home.dart';
import 'package:retronic/tools/game_pad_input_handle.dart';
import 'package:retronic/tools/get_binary_dir.dart';
import 'package:rinf/rinf.dart';

import 'src/bindings/bindings.dart';

Future<void> main() async {
  final hasTinic = await hasTinicIpcInstalled();

  await initializeRust(assignRustSignal);

  if (hasTinic) {
    AppStartSignal(
      tinicIpcFile: await getTinicBinary(),
      baseRetroPath: (await getRetronicDir()).path,
    ).sendSignalToRust();
  }

  runApp(MyApp(hasTinic: hasTinic));
}

class MyApp extends StatefulWidget {
  const MyApp({super.key, required this.hasTinic});

  final bool hasTinic;

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  final gamePadInputObserver = GamePadInputObserver();

  @override
  void initState() {
    gamePadInputObserver.start();
    super.initState();
  }

  @override
  void dispose() {
    gamePadInputObserver.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData.dark(useMaterial3: true),
      home: Visibility(
        visible: widget.hasTinic,
        replacement: DownloadTinicIpc(),
        child: HomePage(),
      ),
    );
  }
}

//
// class MyHomePage extends StatefulWidget {
//   const MyHomePage({super.key});
//
//   @override
//   State<MyHomePage> createState() => _MyHomePageState();
// }
//
// class _MyHomePageState extends State<MyHomePage> {
//   bool closeBtVisibility(GameStateChange state) {
//     switch (state) {
//       case GameStateChange.running:
//         return true;
//       case GameStateChange.closed:
//         return false;
//       case GameStateChange.paused:
//         return true;
//     }
//   }
//
//   @override
//   Widget build(BuildContext context) {
//     return Scaffold(
//       body: Center(
//         child: Column(
//           mainAxisAlignment: .center,
//           children: [
//             const Text('You have pushed the button this many times:'),
//             RetroElevatedButton(
//               onPressed: () {
//                 LoadGame(
//                   romPath: "/home/aderval/Downloads/RetroArch_cores/ff.smc",
//                   corePath:
//                       "/home/aderval/Downloads/RetroArch_cores/RetroArch-Linux-x86_64/RetroArch-Linux-x86_64.AppImage.home/.config/retroarch/cores/snes9x_libretro.so",
//                   baseRetroPath:
//                       "/home/aderval/Downloads/RetroArch_cores/retronic",
//                 ).sendSignalToRust();
//               },
//               child: Text("Jogar"),
//             ),
//
//             StreamBuilder(
//               stream: GameStateChangeSignal.rustSignalStream,
//               builder: (context, snapshot) => Visibility(
//                 visible: snapshot.data?.message.state != null
//                     ? closeBtVisibility(snapshot.data!.message.state)
//                     : false,
//                 child: RetroElevatedButton(
//                   onPressed: () {
//                     CloseGame().sendSignalToRust();
//                   },
//                   child: Text("fechar"),
//                 ),
//               ),
//             ),
//
//             StreamBuilder(
//               stream: SaveStateInfoSignal.rustSignalStream,
//               builder: (context, snapshot) => Visibility(
//                 visible: snapshot.data?.message.saveImgPreview != null,
//                 replacement: Text("sem imagem"),
//                 child: Image.file(
//                   File(snapshot.data?.message.saveImgPreview ?? ""),
//                 ),
//               ),
//             ),
//
//             StreamBuilder(
//               stream: DeviceConnectedSignal.rustSignalStream,
//               builder: (context, snapshot) =>
//                   Text("Controle name: ${snapshot.data?.message.name}"),
//             ),
//           ],
//         ),
//       ),
//     );
//   }
// }
