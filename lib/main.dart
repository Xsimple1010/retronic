import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:retronic/pages/download_tinic_ipc.dart';
import 'package:retronic/pages/home_page.dart';
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

    NeedDownloadSrcSignal().sendSignalToRust();
    GetRecentGamesSignal(page: 1).sendSignalToRust();
  }

  GetRomsFromDirSignal(dir: "/home/aderval/Downloads/roms").sendSignalToRust();

  runApp(ProviderScope(child: MyApp(hasTinic: hasTinic)));
}

class MyApp extends StatefulWidget {
  const MyApp({super.key, required this.hasTinic});

  final bool hasTinic;

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  final gamePadInputObserver = GamePadInputObserver();
  late StreamSubscription<RustSignalPack<OnNeedDownloadSrcSignalOut>>
  downloadSrcSubscription;

  @override
  void initState() {
    gamePadInputObserver.start();

    downloadSrcSubscription = OnNeedDownloadSrcSignalOut.rustSignalStream
        .listen((event) {
          if (!event.message.hasInfo) {
            UpdateInfoSignal(force: true).sendSignalToRust();
          }

          if (!event.message.hasItemInDb) {
            UpdateDatabaseSignal(force: false).sendSignalToRust();
          }
        });

    super.initState();
  }

  @override
  void dispose() {
    gamePadInputObserver.dispose();
    downloadSrcSubscription.cancel();
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
