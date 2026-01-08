import 'package:flutter/material.dart';
import 'package:retronic/tools/game_pad_input_handle.dart';

class RetroIconButton extends StatefulWidget {
  const RetroIconButton({
    super.key,
    required this.onPressed,
    required this.icon,
    this.style,
  });

  final Function() onPressed;
  final Widget icon;
  final ButtonStyle? style;

  @override
  State<RetroIconButton> createState() => _RetroIconButtonState();
}

class _RetroIconButtonState extends State<RetroIconButton> {
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
    return IconButton(
      autofocus: true,
      style: widget.style,
      focusNode: inputObserver.focusNode,
      onPressed: () => widget.onPressed(),
      icon: widget.icon,
    );
  }
}
