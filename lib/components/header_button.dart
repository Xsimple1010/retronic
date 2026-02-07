import 'package:flutter/material.dart';

class HeaderButton extends StatelessWidget {
  const HeaderButton({
    super.key,
    required this.constraints,
    this.onTap,
    this.onHover,
    required this.icon,
  });

  final BoxConstraints constraints;
  final Function()? onTap;
  final Function(bool)? onHover;
  final IconData icon;

  @override
  Widget build(BuildContext context) {
    return IconButton(autofocus: true, icon: Icon(icon), onPressed: onTap);
  }
}
