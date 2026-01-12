import 'dart:io';

import 'package:path_provider/path_provider.dart';

Future<Directory> getRetronicDir() async {
  final docDir = await getApplicationDocumentsDirectory();
  final retronicDir = Directory(
    '${docDir.path}${Platform.pathSeparator}retronic',
  );

  if (!retronicDir.existsSync()) {
    retronicDir.createSync(recursive: true);
  }

  return retronicDir;
}

Future<Directory> getBinaryDir() async {
  final docDir = await getRetronicDir();
  final binaryDir = Directory('${docDir.path}${Platform.pathSeparator}.binary');

  if (!binaryDir.existsSync()) {
    binaryDir.createSync(recursive: true);
  }

  return binaryDir;
}

String tinicBinaryName() {
  if (Platform.isWindows) {
    return 'tinic_ipc.exe';
  } else if (Platform.isLinux) {
    return 'tinic_ipc';
  } else {
    throw UnsupportedError('Plataforma não suportada');
  }
}

Future<bool> hasTinicIpcInstalled() async {
  final binaryDir = await getBinaryDir();

  final tinicIpc = File(
    "${binaryDir.path}${Platform.pathSeparator}${tinicBinaryName()}",
  );

  return tinicIpc.exists();
}

Future<String> getTinicBinary() async {
  final fileName = tinicBinaryName();
  final binaryDir = await getBinaryDir();
  return '${binaryDir.path}${Platform.pathSeparator}$fileName';
}
