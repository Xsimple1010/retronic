import 'package:flutter/widgets.dart';
import 'package:flutter_riverpod/legacy.dart';
import 'package:retronic/src/bindings/signals/signals.dart';
import 'package:rinf/rinf.dart';

class DatabaseProgress {
  int renaming;
  int total;

  DatabaseProgress({required this.renaming, required this.total});
}

class DatabaseUpdateNotifier with ChangeNotifier {
  final DatabaseProgress _databaseItem = DatabaseProgress(
    renaming: 0,
    total: 0,
  );

  DatabaseProgress get databaseProgress => _databaseItem;

  DatabaseUpdateNotifier() {
    // Initialize the notifier
    OnReadRdbProgressOutSignal.rustSignalStream.listen(rdbProgress);
  }

  void rdbProgress(RustSignalPack<OnReadRdbProgressOutSignal> event) {
    final message = event.message;

    _databaseItem.renaming = message.remaining;
    _databaseItem.total = message.total;
    notifyListeners();
  }
}

final databaseProvider = ChangeNotifierProvider<DatabaseUpdateNotifier>((ref) {
  return DatabaseUpdateNotifier();
});
