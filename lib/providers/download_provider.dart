import 'package:flutter/foundation.dart';
import 'package:flutter_riverpod/legacy.dart';
import 'package:retronic/src/bindings/bindings.dart';
import 'package:rinf/rinf.dart';

enum DownloadStatus { pending, inProgress, completed, failed }

class DownloadItem {
  String title;
  double progress;
  DownloadStatus status;

  DownloadItem({
    required this.title,
    required this.progress,
    required this.status,
  });
}

class DownloadNotifier with ChangeNotifier {
  final List<DownloadItem> _downloads = [];

  List<DownloadItem> get downloads => _downloads;

  DownloadNotifier() {
    OnDownloadStartedOutSignal.rustSignalStream.listen(_onDownloadStarted);
    OnDownloadProgressOutSignal.rustSignalStream.listen(_onDownloadProgress);
    OnDownloadCompletedOutSignal.rustSignalStream.listen(_onDownloadCompleted);
  }

  bool allCompleted() {
    return _downloads.every((item) => item.status == DownloadStatus.completed);
  }

  void _onDownloadStarted(RustSignalPack<OnDownloadStartedOutSignal> event) {
    _downloads.add(
      DownloadItem(
        title: event.message.fileName,
        status: DownloadStatus.pending,
        progress: 0.0,
      ),
    );
    notifyListeners();
  }

  void _onDownloadProgress(RustSignalPack<OnDownloadProgressOutSignal> event) {
    final index = _downloads.indexWhere(
      (item) => item.title == event.message.fileName,
    );

    if (index != -1) {
      _downloads[index].title = event.message.fileName;
      _downloads[index].progress = event.message.progress;
      _downloads[index].status = DownloadStatus.inProgress;

      notifyListeners();
    }
  }

  void _onDownloadCompleted(
    RustSignalPack<OnDownloadCompletedOutSignal> event,
  ) {
    final index = _downloads.indexWhere(
      (item) => item.title == event.message.fileName,
    );
    if (index != -1) {
      _downloads[index].title = event.message.fileName;
      _downloads[index].progress = 100.0;
      _downloads[index].status = DownloadStatus.completed;

      notifyListeners();
    }
  }
}

final downloadProvider = ChangeNotifierProvider<DownloadNotifier>((ref) {
  return DownloadNotifier();
});
