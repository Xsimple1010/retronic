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
  FocusNode focusNode = FocusNode();
  Stream<RustSignalPack<DeviceButtonPressedSignal>> buttonPressedOutput =
      DeviceButtonPressedSignal.rustSignalStream;
  late final StreamSubscription _buttonSub;

  void start(Function() onPressed, BuildContext context) {
    _actionHandle(focusNode, onPressed, context);
  }

  void stop() {
    _buttonSub.cancel();
  }

  void _actionHandle(
    FocusNode focusNode,
    Function() onPressed,
    BuildContext context,
  ) {
    _buttonSub = buttonPressedOutput.listen((event) {
      final state = gamePadInputHandle(focusNode, event.message.button);
      if (state == GamePadInputsFocus.click) {
        onPressed();
      } else if (state == GamePadInputsFocus.back) {
        if (context.mounted && Navigator.canPop(context)) {
          Navigator.pop(context);
        }
      }
    });
  }
}
