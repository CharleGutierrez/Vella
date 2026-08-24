# 🏭 Vella: The Ultimate 150 SCADA & Industrial Control Cookbook

For DCS Engineers, Plant Operators, and SCADA Architects running Oil Refineries, Power Grids, or Nuclear Reactors, Vella replaces highly fragmented legacy software with a unified, Rust-powered Control Plane. 

This guide contains **150 progressively complex patterns** for executing native Modbus/OPC UA reads, governing ISA-18.2 Alarm State Machines, and deploying Triple Modular Redundancy (TMR).

---

### 🟢 Part 1: Industrial Protocol Drivers (Modbus & OPC UA) (001 - 025)
**001. Init Modbus TCP Driver:** `let modbus = ScadaDriver::new(IndustrialProtocol::ModbusTcp { ip: "10.0.1.50".to_string(), port: 502 });`
**002. Init OPC UA Driver:** `let opc_ua = ScadaDriver::new(IndustrialProtocol::OpcUa { endpoint_url: "opc.tcp://10.0.1.100:4840".to_string() });`
**003. Read Holding Register (Modbus):** `let pressure_raw = modbus.read_holding_register(40001);`
**004. Read Input Register:** `let temp_raw = modbus.read_input_register(30001);`
**005. Write Single Coil (Valve Actuation):** `modbus.write_coil(00001, true); // DANGER: Opens physical valve`
**006. Write Multiple Registers:** `modbus.write_multiple_registers(40010, vec![100, 200, 300]);`
**007. Read OPC UA Node (Tag):** `let level = opc_ua.read_node("ns=2;s=Tank1.Level");`
**008. Subscribe to OPC UA Node:** `opc_ua.subscribe_node("ns=2;s=Pump1.Vibration", |val| process(val));`
**009. Parse 32-bit Float (Modbus):** `let temp_f32 = f32::from_bits(((reg[0] as u32) << 16) | reg[1] as u32);` // Big-Endian
**010. Handle Endianness Swaps:** `let val_le = f32::from_bits(u32::from_be_bytes([b1, b2, b3, b4]));`
**011. PLC Polling Loop:** `tokio::spawn(async move { loop { poll_plc().await; sleep(100ms).await; } });`
**012. Scale Raw ADC to Engineering Units:** `let pressure_psi = (raw_val as f32 / 4095.0) * 100.0;`
**013. Detect Sensor Disconnect:** `if modbus_err.is_timeout() { trigger_comm_fault_alarm(); }`
**014. Reconnect Strategy:** `if !opc_ua.is_connected() { opc_ua.reconnect().await; }`
**015. Bulk Read Registers (Optimization):** `let block = modbus.read_holding_registers(40001, 100);`
**016. Extract Bits from Word (Alarms):** `let is_high_alarm = (status_word & (1 << 3)) != 0;`
**017. Write Modbus Keep-Alive (Watchdog):** `modbus.write_register(40999, heartbeat_counter);`
**018. Handle Modbus Exceptions:** `if err.code() == 0x02 { log::error!("Illegal Data Address"); }`
**019. Authenticate OPC UA (Certificates):** `opc_ua.set_security_policy(SecurityPolicy::Basic256Sha256);`
**020. Read OPC UA Structs:** `let motor_data: MotorStruct = opc_ua.read_structured_node("ns=2;s=Motor1");`
**021. Execute OPC UA Method:** `opc_ua.call_method("ns=2;s=System", "ns=2;s=System.Reset");`
**022. Verify Device Identity:** `// Compare PLC X.509 certificate against Vella's trusted store`
**023. Map Protocol to Vella DB:** `vella.db.execute("INSERT INTO telemetry (tag, val)...", tag, val);`
**024. Read DNP3 (Power Grid):** `// Utilize DNP3 adapter for Master Station power line telemetry`
**025. Broadcast Tag to UI:** `vella.realtime.broadcast("SCADA_UPDATE", json!({"tag": "T101", "val": 45.2}));`

---

### 🟡 Part 2: ISA-18.2 Alarm State Machines & Compliance (026 - 050)
**026. Init Alarm State Machine:** `let mut alarm = Isa18Alarm::new("PUMP_101_TEMP_HI");`
**027. Define High Threshold:** `const TEMP_HI_SP: f32 = 95.0;`
**028. Trigger Breach (Active/Unack):** `if temp > TEMP_HI_SP { alarm.trigger_breach(); }`
**029. Verify State Transition:** `assert_eq!(alarm.state, AlarmState::UnackActive);`
**030. Operator Acknowledges Alarm:** `alarm.operator_acknowledge();` // Transitions to AckActive
**031. Sensor Returns to Safe Level:** `if temp < TEMP_HI_SP - DEADBAND { alarm.trigger_clear(); }`
**032. Latching Alarm Logic:** `// Alarm remains UnackCleared until physically acknowledged by human`
**033. Deadband / Hysteresis Logic:** `// Prevents alarm toggling on/off rapidly if temp hovers exactly at 95.0`
**034. Alarm Shelving (Maintenance):** `alarm.shelve(Duration::from_hours(4));` // Mutes alarm during repair
**035. Auto-Unshelve Timer:** `if alarm.shelve_time_expired() { alarm.unshelve(); }`
**036. Out-Of-Service (OOS) State:** `// Tagged OOS when the physical sensor is known to be broken`
**037. Alarm Priority Routing (Critical):** `if alarm.priority == Critical { trigger_physical_klaxon(); }`
**038. Alarm Cascade Suppression:** `if main_power.is_down() { suppress_alarms(vec!["PUMP_1", "PUMP_2"]); }`
**039. Prevent Alarm Floods:** `// AI Tuner detects >10 alarms/sec and initiates flood suppression mode`
**040. Write Alarm Log to Audit DB:** `vella.db.execute("INSERT INTO alarm_journal ...", alarm.tag, alarm.state);`
**041. Send SMS to On-Call Engineer:** `if alarm.state == UnackActive && alarm.time_unacked > 5m { send_sms(); }`
**042. Flash UI Red on Unack:** `// Frontend receives WS event; animates background red`
**043. Solid UI Red on Ack:** `// Frontend stops flashing when state transitions to AckActive`
**044. Generate ISA-18.2 KPI Report:** `// Vella calculates Alarm Rate per Hour (Target: < 6 per hour)`
**045. Calculate Stale Alarms:** `// Count alarms that have been Active for > 24 hours without resolution`
**046. Chattering Alarm Detection:** `// Flag an alarm if it transitions > 3 times in 1 minute`
**047. Apply On-Delay Timer:** `if temp > SP for 5s { trigger_breach(); }` // Ignores 1-second spikes
**048. Require Password for Ack:** `// Force re-auth via Vella Auth before acknowledging Critical safety alarms`
**049. Log Acknowledger ID:** `vella.collection('AuditLogs').create({ user: "Operator_John", action: "Ack T101" });`
**050. Route to Plant PA System:** `// Execute text-to-speech SIP call to plant intercom on catastrophic alarm`

---

### 🟠 Part 3: Enterprise Historians & Analog Compression (051 - 075)
**051. Init Swinging Door Compressor:** `let mut compressor = SwingingDoorCompressor::new(0.5);`
**052. Process High-Hz Signal:** `if let Some(val) = compressor.process_signal(temp_raw, disk_usage) { save(val); }`
**053. Drop Redundant Data:** `// If temp holds at 100.0, 100.1, 100.0... compressor drops 99% of packets`
**054. Break Geometry / Archive Point:** `// If temp spikes to 101.5, compressor archives it, saving the curve`
**055. Tune Threshold Dynamically:** `// Vella AiTuner detects 95% disk usage; widens compression tolerance to 2.0`
**056. Init TimescaleDB Backend:** `let ts = TimeSeriesAdapter::new("TimescaleDB", ai_tuner);`
**057. Write to Hypertable:** `vella.db.execute("INSERT INTO historian (time, tag, value) VALUES (NOW(), ?, ?)");`
**058. Downsample for Dashboard:** `let sql = ts.query_downsampled_bucket("reactor_temp", 1000, 10);` // 1s buckets
**059. Calculate Rate of Change (RoC):** `SELECT (last(val, time) - first(val, time)) / 60 as roc_per_minute`
**060. Linear Interpolation (Filling gaps):** `// Fill missing seconds in the UI graph using SQL interpolation`
**061. Last-Value-Carried-Forward (LOCF):** `// Standard SCADA graphing method for step-change signals (e.g. Setpoints)`
**062. Calculate Totalized Flow:** `SELECT sum(flow_rate * delta_t) AS total_gallons FROM flow_meter;`
**063. Daily Shift Reporting:** `// Cron job runs at 06:00 to generate PDF report of total production vs targets`
**064. Query Max Historical Value:** `// Fast query across TimescaleDB chunks to find all-time max pressure`
**065. Data Retention Policy (Drop Old):** `SELECT add_retention_policy('historian', INTERVAL '5 years');`
**066. Continuous Aggregation:** `// Vella natively maps an hourly materialized view to keep multi-year queries fast`
**067. Store Complex Blob Data:** `// Map raw vibration Fast Fourier Transform (FFT) arrays into PostgreSQL BYTEA`
**068. Predictive Failure (Data Science):** `// Python fetches Pandas Arrow IPC stream from Vella Historian for ML training`
**069. Detect Pump Cavitation:** `// Vella Wasm edge module analyzes vibration variance locally and flags anomaly`
**070. Monitor Historian Queue Size:** `// Vella exposes internal metric of pending DB inserts to prevent memory OOM`
**071. Export Data for Regulatory Audit:** `let csv = vella.data.export_to_csv_stream("SELECT * WHERE time > ...");`
**072. Timezone Handling (UTC):** `// All Vella backend historian timestamps are strictly UTC. Shift adjusted on UI.`
**073. Cold Storage Archiving:** `// Vella background job moves data > 1 year old from SSDs to cheap HDD arrays`
**074. Calculate Equipment Uptime:** `SELECT sum(duration) WHERE state = 'RUNNING' GROUP BY pump_id;`
**075. Overall Equipment Effectiveness (OEE):** `let oee = availability * performance * quality;`

---

### 🔴 Part 4: Triple Modular Redundancy (TMR) & Hardware Voting (076 - 100)
**076. Init TMR Voter:** `// Critical safety module for Nuclear / Chemical plants`
**077. Read 3 Physical Sensors:** `let (s1, s2, s3) = (read_a(), read_b(), read_c());`
**078. Execute Hardware Vote:** `let safe_val = TmrVoter::execute_hardware_vote(s1, s2, s3).unwrap();`
**079. Handle Node Divergence (A!=B):** `// TmrVoter detects Node B output 0 while A & C output 1. Returns 1.`
**080. Isolate Faulty Hardware:** `if s2_faulty { vella.db.execute("UPDATE hardware SET status='ISOLATED' WHERE id='B'"); }`
**081. Degrade to 1oo2 Voting:** `// If Node B is offline, logic drops to 1-out-of-2 mode (A & C must agree)`
**082. Total Consensus Failure:** `if a != b && b != c { trigger_emergency_shutdown(); }`
**083. Network Heartbeat (Cluster):** `// Vella continuously pings Server A, B, and C over separate NICs`
**084. Split-Brain Detection:** `// If Server A loses connection to B & C, it voluntarily steps down (Suicide mode)`
**085. Hot-Standby Failover:** `if primary_dead { promote_secondary_to_active(); }`
**086. Synchronize State to Standby:** `// Vella ships WAL stream to Standby node over dedicated crossover cable`
**087. TMR Analog Tolerance Voting:** `// For floats: if A, B, C are within 0.1% of each other, average them`
**088. Execute SCRAM (Nuclear Shutdown):** `RtosIsolator::spawn_hard_realtime_task("SCRAM", || drop_control_rods());`
**089. Lock Critical I/O Threads:** `// Ensure SCRAM thread runs at OS Priority 99 (highest realtime scheduling)`
**090. Prevent Priority Inversion:** `// Use lock-free Atomic types to pass data between Web threads and RTOS threads`
**091. Safety Instrumented System (SIS):** `// Decouple standard SCADA control logic from dedicated SIS Safety logic`
**092. Validate Command Echo:** `// Tell PLC to open valve, then read limit switch to verify it physically opened`
**093. Detect Valve Stuck:** `if cmd_open && !limit_switch_open_after_5s { trigger_valve_stuck_alarm(); }`
**094. Interlock Logic (Permissives):** `if !pump_running || pressure < MIN { block_heater_startup(); }`
**095. Verify Watchdog Relays:** `// Physical relays stay closed as long as Vella toggles a GPIO pin every 10ms`
**096. Fail-Safe State Output:** `if vella_crashes { hardware_relays_open_automatically(); }` // Physical gravity shutdown
**097. Compare Vella vs Legacy DCS:** `// Shadow Route logic: Run Vella alongside Siemens DCS to verify matching outputs`
**098. Log Voting Discrepancies:** `vella.collection('TmrLogs').create({ node: "C", expected: 1, actual: 0 });`
**099. Require Physical Key Switch:** `// Bypass software limits only if physical maintenance key is turned on panel`
**100. Execute Periodic Proof Test:** `// Autonomous sequence to pulse a valve 5% closed to prove it isn't seized`

---

### 🟣 Part 5: HMI Canvas & Real-Time Dashboards (101 - 125)
**101. Init HMI Canvas Builder:** `let hmi = HmiCanvasBuilder::new();`
**102. Bind SVG to Telemetry Tag:** `let binding = hmi.bind_svg_to_telemetry_tag("tank_1", "LIQUID_LVL");`
**103. Send Binding to Frontend:** `vella.realtime.broadcast("HMI_INIT", json!(binding));`
**104. Animate Tank Fill (React/Vue):** `// Frontend sets SVG <rect height="val"> based on live WebSocket stream`
**105. Dynamic Color States:** `// Green < 80%, Yellow > 80%, Red > 95% driven by HmiCanvas logic`
**106. Render P&ID Topologies:** `// Load Piping and Instrumentation Diagram (SVG) and bind 100+ tags programmatically`
**107. Render Rotating Pump (SVG):** `// Apply CSS transform: rotate() based on RUNNING boolean tag`
**108. Render Flashing Flame:** `if fire_detected { toggle_svg_visibility(true); }`
**109. HMI Command Output:** `// Operator clicks SVG Valve -> Vella API -> Modbus -> Physical Valve opens`
**110. Sparkline Trend Graphs:** `// Small SVG line charts rendered under sensors showing last 10 minutes of history`
**111. High-Performance HMI Standard:** `// Use muted greys/blues for normal operation. Only use Bright Colors for Alarms.`
**112. Multi-Monitor Control Room:** `// Open separate Vella SPA routes: /hmi/overview, /hmi/alarms, /hmi/trends`
**113. Synchronize UI Time to Server:** `// All HMI clients display strict Server UTC time to prevent timezone confusion`
**114. Display Network Health:** `// Green/Red indicator on HMI showing Vella-to-PLC ping latency`
**115. Pop-Up Faceplate (Faceplate UI):** `// Clicking a pump opens a detailed React modal with Manual/Auto mode toggles`
**116. Switch to Manual Mode:** `vella.db.execute("UPDATE control_loops SET mode='MANUAL' WHERE id='PID_1'");`
**117. Adjust PID Setpoint (SP):** `vella.scada.write_register(40100, new_setpoint_value);`
**118. Monitor PID Process Variable (PV):** `// Read current actual temperature`
**119. Monitor PID Control Output (CV):** `// Read 0-100% output to the physical heater element`
**120. HMI Inactivity Logout:** `if !mouse_moved_15m { vella.auth.logout(); }`
**121. Dark Mode / Control Room Mode:** `// CSS theme tailored for dark control rooms to reduce operator eye fatigue`
**122. Pan & Zoom Canvas:** `// Allow operator to zoom into complex SVG refinery pipelines via d3.js hooks`
**123. Decluttering on Zoom:** `if zoom_level < 50% { hide_minor_labels(); }` // Semantic zooming
**124. Display Live Camera Feed:** `// Embed RTSP/HLS stream from plant security cameras directly in the HMI dashboard`
**125. Global Acknowledge Button:** `// Single button to acknowledge all visible alarms on the current screen`

---

### 🔵 Part 6: Security, Air-Gapping & Enterprise Governance (126 - 150)
**126. Run Vella 100% Offline (Air-Gapped):** `// No external API calls. Cargo build --release deployed to physical server`
**127. Local LLM for Safety Manuals:** `let llm = LocalLlmEngine::new("./models/llama3.gguf");` // Runs strictly on local server
**128. RAG Search Plant Procedures:** `let proc = llm.query("What is the protocol for secondary loop pressure drop?");`
**129. Restrict LLM Hallucinations:** `// Vella Semantic Cache forces LLM to cite specific PDF page numbers in responses`
**130. Multi-Signature Setpoint Change:** `// Operator requests Setpoint increase. Requires Shift Supervisor password to execute.`
**131. Reject Unapproved Commands:** `if !record.status == 'APPROVED' { block_modbus_write(); }`
**132. Deep RBAC Isolation:** `// Engineers can view Unit A, but cannot issue writes to Unit B`
**133. Record Audit Trail (21 CFR Part 11):** `// Pharmaceutical-grade logging. Who clicked what, when, old value, new value.`
**134. Cryptographic Log Hashing:** `// Hash the audit log table continuously so operators cannot manually delete mistakes`
**135. Disable USB Ports (OS Level):** `// Server hardened physically; Vella deployed via secure encrypted SSH tunnel`
**136. Monitor PLC Logic Changes:** `// Vella periodically checksums the PLC ladder logic to detect Stuxnet-style malware`
**137. Block Anomalous Commands:** `if setpoint > 500.0 { reject("Command exceeds physical safety limits"); }`
**138. Enforce Shift Schedules:** `if current_time > shift_end { force_operator_logout(); }`
**139. Validate Data Integrity (Checksums):** `if packet_checksum != calculated { drop_telemetry_packet(); }`
**140. Execute Cyber-Attack Drill:** `vella.chaos.triggerPartition();` // Test if operators can handle loss of visibility safely
**141. Active Directory (LDAP) Auth:** `// Integrate Vella Auth with Plant-wide Microsoft Active Directory`
**142. Biometric Auth (Smart Cards):** `// Map Vella login to RFID Badge scans at the control room console`
**143. Map IP Addresses to Subnets:** `// Reject Vella API calls originating from outside the strict Control Network subnet`
**144. Disconnect from Business Network:** `// Enforce unidirectional data diode (Waterfall) between SCADA network and Corporate`
**145. Export Read-Only Data to Diode:** `// Send UDP telemetry through one-way fiber optic cable to Corporate analytics database`
**146. Detect Denial of Service (DoS):** `// Vella clamps API limits if corporate network attempts to flood SCADA server`
**147. Print Physical Shift Report:** `// Auto-generate PDF and send to physical plant printer via IPP`
**148. Deploy to Ruggedized Edge IPC:** `// Vella compiled for ARM64 to run on fanless DIN-rail mounted industrial PCs`
**149. Survive Power Fluctuation:** `// Vella SQLite uses WAL mode to guarantee zero DB corruption if power cord is pulled`
**150. Safe Mode Initialization:** `// On reboot, Vella verifies all hardware states before re-enabling manual control outputs`
