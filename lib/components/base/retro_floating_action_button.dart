import 'package:flutter/material.dart';
import 'package:retronic/tools/game_pad_input_handle.dart';

class RetroFloatingActionButton extends StatefulWidget {
  const RetroFloatingActionButton({
    super.key,
    required this.onPressed,
    this.child,
  });

  final void Function() onPressed;
  final Widget? child;

  @override
  State<RetroFloatingActionButton> createState() =>
      _RetroFloatingActionButtonState();
}

class _RetroFloatingActionButtonState extends State<RetroFloatingActionButton> {
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

  @override
  Widget build(BuildContext context) {
    return FloatingActionButton(
      autofocus: true,
      focusNode: inputObserver.focusNode,
      onPressed: widget.onPressed,
      child: widget.child,
    );
  }
}
