import 'package:flutter/material.dart';

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
  @override
  Widget build(BuildContext context) {
    return FloatingActionButton(
      autofocus: true,
      onPressed: widget.onPressed,
      child: widget.child,
    );
  }
}
