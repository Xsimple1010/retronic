import 'package:flutter/material.dart';
import 'package:retronic/tools/game_pad_input_handle.dart';

class RetroElevatedButton extends StatefulWidget {
  const RetroElevatedButton({
    super.key,
    required this.child,
    this.onHove,
    required this.onPressed,
    this.onLongPress,
    this.onFocusChange,
    this.style,
  });

  final Widget child;

  final void Function(bool)? onHove;
  final Function() onPressed;
  final void Function()? onLongPress;
  final Function(bool)? onFocusChange;
  final ButtonStyle? style;

  @override
  State<RetroElevatedButton> createState() => _RetroElevatedButtonState();
}

class _RetroElevatedButtonState extends State<RetroElevatedButton> {
  final GamePadInputObserver inputObserver = GamePadInputObserver();

  @override
  void initState() {
    inputObserver.start(widget.onPressed, context);
    super.initState();
  }

  @override
  void dispose() {
    inputObserver.stop();
    super.dispose();
  }

  void onPressHandle() {
    setState(() {
      inputObserver.focusNode.requestFocus();
    });

    widget.onPressed();
  }

  @override
  Widget build(BuildContext context) {
    return ElevatedButton(
      autofocus: true,
      focusNode: inputObserver.focusNode,
      style: widget.style,
      onFocusChange: (value) => {
        if (widget.onFocusChange != null) {widget.onFocusChange!(value)},
      },
      onHover: (value) {
        if (widget.onHove != null) {
          setState(() {
            inputObserver.focusNode.requestFocus();
          });
          widget.onFocusChange!(value);
        }
      },
      onPressed: onPressHandle,
      child: widget.child,
    );
  }
}
