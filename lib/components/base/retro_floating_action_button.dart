import 'package:flutter/material.dart';
import 'package:retronic/src/bindings/bindings.dart';
import 'package:retronic/tools/game_pad_input_handle.dart';

class RetroFloatingActionButton extends StatefulWidget {
  const RetroFloatingActionButton({
    super.key,
    this.onPressed,
    this.child,
  });

  final void Function()? onPressed;
  final Widget? child;

  @override
  State<RetroFloatingActionButton> createState() =>
      _RetroFloatingActionButtonState();
}

class _RetroFloatingActionButtonState extends State<RetroFloatingActionButton> {
  final buttonPressedOutput = DeviceButtonPressedSignal.rustSignalStream;
  final FocusNode focusNode = FocusNode();

  @override
  void initState() {
    buttonPressedOutput.listen((event) {
      final state = gamePadInputHandle(focusNode, event.message.name);
      if (state == GamePadInputsFocus.click) {
        if (widget.onPressed != null) {
          widget.onPressed!();
        }
      } else if (state == GamePadInputsFocus.back) {
        if(context.mounted) {
          Navigator.pop(context);
        }
      }
    });
    super.initState();
  }

  @override
  void dispose() {
    focusNode.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return FloatingActionButton(
      autofocus: true,
      focusNode: focusNode,
      onPressed: widget.onPressed,
      child: widget.child,
    );
  }
}
