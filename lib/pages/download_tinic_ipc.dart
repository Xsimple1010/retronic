import 'package:flutter/material.dart';
import 'package:retronic/pages/home_page.dart';
import 'package:retronic/src/bindings/bindings.dart';
import 'package:retronic/tools/get_binary_dir.dart';
import 'package:retronic/tools/tinic_ipc_manage.dart';

class DownloadTinicIpc extends StatefulWidget {
  const DownloadTinicIpc({super.key});

  @override
  State<DownloadTinicIpc> createState() => _DownloadTinicIpcState();
}

class _DownloadTinicIpcState extends State<DownloadTinicIpc> {
  @override
  void initState() {
    downloadTinicIpc(version: "v1.0.3", onProgress: onProgress);
    super.initState();
  }

  Future<void> onProgress(double progress) async {
    if (progress >= 1.0) {
      if (!context.mounted) {
        return;
      }

      AppStartSignal(
        tinicIpcFile: await getTinicBinary(),
        baseRetroPath: (await getRetronicDir()).path,
      ).sendSignalToRust();

      Navigator.pushReplacement(
        context,
        MaterialPageRoute(builder: (_) => HomePage()),
      );
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: LayoutBuilder(
        builder: (context, constraints) => Stack(
          fit: StackFit.expand,
          alignment: Alignment.bottomCenter,
          children: [
            Positioned(
              bottom: constraints.maxHeight * .5,
              child: Column(
                children: [
                  Text(
                    "BAIXANDO CONTEÚDO ADICIONAL",
                    style: TextStyle(
                      fontSize: constraints.maxHeight * .06,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                  Text(
                    "O RETRONIC DEPENDO DO TINIC_IPC PARA FUNCIONA, POR-FAVOR ESPERE UM POUCO!",
                    style: TextStyle(
                      fontSize: constraints.maxHeight * .022,
                      fontWeight: FontWeight.w300,
                    ),
                  ),
                ],
              ),
            ),

            Positioned(
              bottom: constraints.maxHeight * .1,
              child: SizedBox(
                height: 10,
                width: 400,
                child: LinearProgressIndicator(
                  color: Colors.blue,
                  borderRadius: BorderRadius.circular(12),
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
