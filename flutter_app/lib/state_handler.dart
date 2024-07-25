import 'dart:async';

import 'package:flutter/foundation.dart';
import 'package:esp_server_app/rust/api/state.dart';

class StateHandler extends ChangeNotifier {
  static final StateHandler _instance = StateHandler._();

  State _state = State();
  late StreamSubscription<State> _stateSubscription;

  factory StateHandler() {
    return _instance;
  }

  StateHandler._() {
    _stateSubscription = initStateSink().listen((state) {
      _state = state;
      notifyListeners();
    });
  }

  @override
  void dispose() {
    _stateSubscription.cancel();
    super.dispose();
  }

  bool get btConnected => _state.btConnected;
  String get ip => _state.ip;
}
