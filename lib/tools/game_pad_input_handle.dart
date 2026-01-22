import 'dart:async';

import 'package:flutter/material.dart';
import 'package:retronic/src/bindings/bindings.dart';
import 'package:rinf/rinf.dart';

enum GamePadInputsFocus { none, click, back }

// return true if widget is pressed
GamePadInputsFocus gamePadInputHandle(FocusNode focusNode, String name) {
  if (focusNode.hasFocus) {
    if (name == "DPad-down") {
      focusNode.focusInDirection(TraversalDirection.down);
    } else if (name == "DPad-up") {
      focusNode.focusInDirection(TraversalDirection.up);
    } else if (name == "DPad-left") {
      focusNode.focusInDirection(TraversalDirection.left);
    } else if (name == "DPad-right") {
      focusNode.focusInDirection(TraversalDirection.right);
    } else if (name == "B") {
      return GamePadInputsFocus.click;
    } else if (name == "A") {
      return GamePadInputsFocus.back;
    }
  }

  return GamePadInputsFocus.none;
}

class GamePadInputObserver {
  Stream<RustSignalPack<OnDeviceButtonPressedOutSignal>> buttonPressedOutput =
      OnDeviceButtonPressedOutSignal.rustSignalStream;

  late final StreamSubscription _buttonSub;

  void start() {
    _actionHandle();
  }

  void dispose() {
    _buttonSub.cancel();
  }

  void _actionHandle() {
    _buttonSub = buttonPressedOutput.listen((event) {
      final focusNode = FocusManager.instance.primaryFocus;

      if (focusNode == null) {
        return;
      }

      final context = focusNode.context!;

      if (!context.mounted) {
        return;
      }

      final state = gamePadInputHandle(focusNode, event.message.button);

      if (state == GamePadInputsFocus.click) {
        Actions.invoke(context, const ActivateIntent());
      } else if (state == GamePadInputsFocus.back) {
        if (Navigator.canPop(context)) {
          Navigator.pop(context);
        }
      }
    });
  }
}
