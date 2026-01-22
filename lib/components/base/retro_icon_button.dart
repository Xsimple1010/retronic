import 'package:flutter/material.dart';

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

  @override
  Widget build(BuildContext context) {
    return IconButton(
      autofocus: true,
      style: widget.style,
      onPressed: () => widget.onPressed(),
      icon: widget.icon,
    );
  }
}
