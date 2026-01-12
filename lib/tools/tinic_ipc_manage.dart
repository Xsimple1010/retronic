import 'dart:io';

import 'package:dio/dio.dart';
import 'package:retronic/tools/get_binary_dir.dart';

String tinicDownloadUrl(String version) {
  if (Platform.isWindows) {
    return 'https://github.com/Xsimple1010/tinic/releases/download/$version/tinic_ipc.exe';
  } else if (Platform.isLinux) {
    return 'https://github.com/Xsimple1010/Tinic/releases/download/$version/tinic_ipc';
  } else {
    throw UnsupportedError('Plataforma não suportada');
  }
}

Future<File> downloadTinicIpc({
  required String version,
  required Future<void> Function(double progress)? onProgress,
}) async {
  final dio = Dio();

  final filePath = await getTinicBinary();

  final file = File(filePath);

  await dio.download(
    tinicDownloadUrl(version),
    filePath,
    onReceiveProgress: (received, total) {
      if (total > 0 && onProgress != null) {
        onProgress(received / total);
      }
    },
    options: Options(
      responseType: ResponseType.bytes,
      followRedirects: true,
      receiveTimeout: const Duration(minutes: 5),
    ),
  );

  if (Platform.isLinux) {
    await Process.run('chmod', ['+x', filePath]);
  }

  return file;
}
