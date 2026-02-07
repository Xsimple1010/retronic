import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:retronic/providers/notification_provider.dart';

class DownlaodPage extends ConsumerWidget {
  const DownlaodPage({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final notification = ref.watch(notificationProvider);

    return Scaffold(
      appBar: AppBar(actions: [BackButton()]),
      body: ListView.builder(
        itemCount: notification.notifications.length,
        itemBuilder: (context, index) => Column(
          children: [
            Text(notification.notifications[index].id),
            Text(notification.notifications[index].message),
            LinearProgressIndicator(
              value: notification.notifications[index].progress / 100,
            ),
          ],
        ),
      ),
    );
  }
}
