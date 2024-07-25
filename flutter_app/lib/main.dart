import 'dart:io';

import 'package:bluetooth_enable_fork/bluetooth_enable_fork.dart';
import 'package:flutter/material.dart';
import 'package:permission_handler/permission_handler.dart';
import 'package:provider/provider.dart';
import 'package:esp_server_app/ble_setup.dart';
import 'package:esp_server_app/rust/api/setup.dart';
import 'package:esp_server_app/rust/api/ble.dart';
import 'package:esp_server_app/rust/frb_generated.dart';
import 'package:esp_server_app/state_handler.dart';
import 'package:esp_server_app/wifi_input.dart';

String level(Level lvl) {
  switch (lvl) {
    case Level.debug:
      return 'DBG';
    case Level.error:
      return 'ERR';
    case Level.info:
      return 'INF';
    case Level.trace:
      return 'TRC';
    case Level.warn:
      return 'WRN';
    default:
      return 'UNKNOWN';
  }
}

Future<void> checkPerm() async {
  if (!Platform.isAndroid) {
    print("Not checking permissions");
    return;
  }
  print("Checking permissions");
  var permissions = [
    Permission.bluetoothScan,
    Permission.bluetoothConnect,
    Permission.location,
  ];
  for (var perm in permissions) {
    final status = await perm.request();
    print("Permission $perm: $status");
    if (await perm.status.isPermanentlyDenied) {
      await openAppSettings();
    }
  }
  var enabled = await BluetoothEnable.enableBluetooth;
  if (enabled == "false") {
    print("bluetooth not enabled");
  }
}

Future<void> main() async {
  await RustLib.init();
  WidgetsFlutterBinding.ensureInitialized();
  await checkPerm();
  setupLogStream().listen((entry) {
    print('Rust: [${level(entry.logLevel)}] ${entry.msg}');
  });
  await setupBle();
  runApp(
    ChangeNotifierProvider(
      create: (context) => StateHandler(),
      child: const MyApp(),
    ),
  );
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: appBar('Esp Server'),
        body: const Home(),
      ),
    );
  }
}

PreferredSizeWidget appBar(String title) {
  return AppBar(
    title: Text(title),
    actions: [
      Consumer<StateHandler>(
        builder: (ctx, state, _) => IconButton(
          icon: state.btConnected
              ? const Icon(Icons.bluetooth, color: Colors.blue)
              : const Icon(Icons.bluetooth_disabled),
          onPressed: () {
            Navigator.push(
              ctx,
              MaterialPageRoute(builder: (context) {
                return const BleSetup();
              }),
            );
          },
        ),
      ),
      const SizedBox(width: 30),
    ],
  );
}

class Home extends StatelessWidget {
  const Home({super.key});

  @override
  Widget build(BuildContext context) {
    return const WifiInput();
  }
}
