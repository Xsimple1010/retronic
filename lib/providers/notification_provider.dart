import 'package:flutter/material.dart';
import 'package:flutter_riverpod/legacy.dart';
import 'package:retronic/providers/database_update_provider.dart';
import 'package:retronic/providers/download_provider.dart';
import 'package:retronic/providers/extraction_provider.dart';

enum AppNotificationType {
  databaseProgress,
  extractionProgress,
  downloadStarted,
  downloadCompleted,
  downloadInProgress,
  extractionCompleted,
  // allDownloadsCompleted,
}

class AppNotification {
  final AppNotificationType type;
  final String id;
  String message;
  double progress;
  final DateTime timestamp;

  AppNotification({
    required this.progress,
    required this.type,
    required this.message,
    required this.id,
  }) : timestamp = DateTime.now();
}

class NotificationNotifier extends ChangeNotifier {
  final List<AppNotification> _notifications = [];

  List<AppNotification> get notifications => List.unmodifiable(_notifications);

  void add(AppNotification notification) {
    final index = _notifications.indexWhere((n) => n.id == notification.id);

    if (index != -1) {
      _notifications[index] = notification;
    } else {
      _notifications.insert(0, notification);
    }

    notifyListeners();
  }

  void clear() {
    _notifications.clear();
    notifyListeners();
  }
}

final notificationProvider = ChangeNotifierProvider<NotificationNotifier>((
  ref,
) {
  final notifier = NotificationNotifier();

  // 🔹 Escuta progresso do banco
  ref.listen<DatabaseUpdateNotifier>(databaseProvider, (prev, next) {
    final db = next.databaseProgress;

    notifier.add(
      AppNotification(
        id: "RDB",
        type: AppNotificationType.databaseProgress,
        message: "Processando RDB: ${db.renaming}/${db.total}",
        progress: ((db.total - db.renaming) / db.total) * 100,
      ),
    );
  });

  // 🔹 Escuta extração de arquivos
  ref.listen<ExtractionNotifier>(extractionProvider, (prev, next) {
    final extraction = next.extractedItems;

    if (extraction.isEmpty) return;

    final last = extraction.last;

    notifier.add(
      AppNotification(
        id: last.originFile,
        type: last.status == ExtractionStatus.completed
            ? AppNotificationType.extractionCompleted
            : AppNotificationType.extractionProgress,
        message: last.status == ExtractionStatus.completed
            ? "Extraiu arquivo: ${last.lastInnerFileName}"
            : "Extraindo arquivo: ${last.lastInnerFileName}",
        progress: last.status == ExtractionStatus.completed ? 100.0 : 0.0,
      ),
    );
  });

  // 🔹 Escuta downloads
  ref.listen<DownloadNotifier>(downloadProvider, (prev, next) {
    final downloads = next.downloads;

    if (downloads.isEmpty) return;

    final last = downloads.last;

    switch (last.status) {
      case DownloadStatus.pending:
        notifier.add(
          AppNotification(
            id: last.title,
            type: AppNotificationType.downloadStarted,
            message: "Download iniciado: ${last.title}",
            progress: 0.0,
          ),
        );
        break;

      case DownloadStatus.completed:
        notifier.add(
          AppNotification(
            id: last.title,
            type: AppNotificationType.downloadCompleted,
            message: "Download concluído: ${last.title}",
            progress: last.progress,
          ),
        );
        break;
      case DownloadStatus.inProgress:
        notifier.add(
          AppNotification(
            id: last.title,
            type: AppNotificationType.downloadInProgress,
            message: "Download em andamento: ${last.title}",
            progress: last.progress,
          ),
        );
        break;

      default:
        break;
    }

    // if (next.allCompleted()) {
    //   notifier.add(
    //     AppNotification(
    //       type: AppNotificationType.allDownloadsCompleted,
    //       message: "Todos downloads finalizados",
    //     ),
    //   );
    // }
  });

  return notifier;
});
