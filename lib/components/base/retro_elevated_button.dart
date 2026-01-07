import 'package:flutter/material.dart';
import 'package:retronic/src/bindings/bindings.dart';
import 'package:retronic/tools/game_pad_input_handle.dart';

class RetroElevatedButton extends StatefulWidget {
  const RetroElevatedButton({
    super.key,
    required this.child,
    this.onHove,
    this.onPressed,
    this.onLongPress,
    this.onFocusChange,
    this.style,
  });

  final Widget child;

  final void Function(bool)? onHove;
  final Function()? onPressed;
  final void Function()? onLongPress;
  final Function(bool)? onFocusChange;
  final ButtonStyle? style;

  @override
  State<RetroElevatedButton> createState() => _RetroElevatedButtonState();
}

class _RetroElevatedButtonState extends State<RetroElevatedButton> {
  final buttonPressedOutput = DeviceButtonPressedSignal.rustSignalStream;
  final focusNode = FocusNode();

  void onPressHandle() {
    if (widget.onPressed != null) {
      setState(() {
        focusNode.requestFocus();
      });
      widget.onPressed!();
    }
  }

  @override
  void initState() {
    buttonPressedOutput.listen((event) {
      final state = gamePadInputHandle(focusNode, event.message.button);
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
    return ElevatedButton(
      autofocus: true,
      focusNode: focusNode,
      style: widget.style,
      onFocusChange: (value) => {
        if (widget.onFocusChange != null) {widget.onFocusChange!(value)}
      },
      onHover: (value) {
        if (widget.onHove != null) {
          setState(() {
            focusNode.requestFocus();
          });
          widget.onFocusChange!(value);
        }
      },
      onPressed: onPressHandle,
      child: widget.child,
    );
  }
}
