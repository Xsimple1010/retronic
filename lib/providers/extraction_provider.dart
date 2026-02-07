import 'package:flutter/foundation.dart';
import 'package:flutter_riverpod/legacy.dart';
import 'package:retronic/src/bindings/bindings.dart';
import 'package:rinf/rinf.dart';

enum ExtractionStatus { pending, completed }

class ExtractedItem {
  final String originFile;
  String lastInnerFileName;
  ExtractionStatus status;

  ExtractedItem({
    required this.originFile,
    required this.lastInnerFileName,
    this.status = ExtractionStatus.pending,
  });
}

class ExtractionNotifier with ChangeNotifier {
  final List<ExtractedItem> _extractedItems = [];

  List<ExtractedItem> get extractedItems => _extractedItems;

  ExtractionNotifier() {
    OnExtractingOutSignal.rustSignalStream.listen(onExtracting);
    OnExtractFinishedOutSignal.rustSignalStream.listen(onExtractFinished);
  }

  void onExtracting(RustSignalPack<OnExtractingOutSignal> event) {
    final index = _extractedItems.indexWhere(
      (item) => item.originFile == event.message.originFile,
    );

    if (index == -1) {
      _extractedItems.add(
        ExtractedItem(
          originFile: event.message.originFile,
          lastInnerFileName: event.message.innerFileName,
        ),
      );
    } else {
      _extractedItems[index].status = ExtractionStatus.pending;
      _extractedItems[index].lastInnerFileName = event.message.innerFileName;
    }

    notifyListeners();
  }

  void onExtractFinished(RustSignalPack<OnExtractFinishedOutSignal> event) {
    final index = _extractedItems.indexWhere(
      (item) => item.originFile == event.message.originFile,
    );

    if (index != -1) {
      _extractedItems[index].status = ExtractionStatus.completed;
    }

    notifyListeners();
  }
}

final extractionProvider = ChangeNotifierProvider<ExtractionNotifier>((ref) {
  return ExtractionNotifier();
});
