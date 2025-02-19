use serde::{Serialize, Deserialize};

// Realtime Status Structure
#[derive(Serialize, Deserialize, Debug)]
pub struct RealtimeStatus {
    pub fryST: u8,      // Frying State (0-6, see enums)
    pub prbT: i16,      // Probe Temp (in 1-degree increments)
    pub prbT10: i16,    // Probe Temp (in 0.1-degree increments)
    pub cjT: i16,       // Board Temp (in 1-degree increments)
    pub cjT10: i16,     // Board Temp (in 0.1-degree increments)
    pub curRcp: u8,     // Current Recipe (0-108)
    pub curGrp: u8,     // Current Group (0-5)
    pub curSP: i16,     // Current Setpoint (degrees F)
    pub curLvl: u8,     // Current User Level (16-19, see enums)
}

// Recipe Structure
#[derive(Serialize, Deserialize, Debug)]
pub struct Recipe {
    pub rName: String,  // Recipe Name (19 characters + /0 null terminator)
    pub rTime: u16,     // Recipe Time (60-28800 seconds)
    pub rTemp: u16,     // Recipe Temp (200-525 degrees F)
    pub rIcon: u8,      // Recipe Icon Index
    pub rBlwr: u8,      // Blower Speed (176 = low, 177 = high)
    pub rGrp: u8,       // Recipe Group (0-5)
    pub rSpare: u8,     // Spare (Unused)
    pub rCRC: u32,      // Checksum (32-bit CRC)
}

// Group Structure
#[derive(Serialize, Deserialize, Debug)]
pub struct Group {
    pub gName: String,  // Group Name (11 characters + /0 null terminator)
    pub gIcon: u16,     // Group Icon Index
    pub gSpare: u16,    // Spare (Unused)
    pub gCRC: u32,      // Checksum (32-bit CRC)
}

// Controller Configuration Structure
#[derive(Serialize, Deserialize, Debug)]
pub struct ControllerConfig {
    pub bVol: u8,       // Buzzer volume (Unused)
    pub tUnits: String, // Temperature Units ("C" or "F" for display)
    pub cSpare1: u16,   // Spare (Unused)
    pub minTime: u16,   // Minimum fry time (seconds)
    pub maxTime: u16,   // Maximum fry time (seconds)
    pub minTemp: u16,   // Minimum fry temp (degrees F)
    pub maxTemp: u16,   // Maximum fry temp (degrees F)
    pub hTemp: u16,     // Hold temp (degrees F)
    pub hTime: u16,     // Hold time (seconds)
    pub cHold: u16,     // CAT hold seconds (fixed in firmware, no longer used)
    pub inpCal: i16,    // Input calibration (signed, in 0.1-degree F)
    pub enbPid: bool,   // Enable PID (Always true)
    pub blEnb: bool,    // Blower enable (Always true)
    pub blSpd: u8,      // Blower speeds (1 = single speed, 2 = dual speed)
    pub cSpare2: u8,    // Spare (Unused)
    pub kpTerm: f32,    // PID kP term (floating point)
    pub kiTerm: f32,    // PID kI term (floating point)
    pub kdTerm: f32,    // PID kD term (floating point)
    pub cCRC: u32,      // Checksum (32-bit CRC)
}

// Log Event Structure
#[derive(Serialize, Deserialize, Debug)]
pub struct LogEvent {
    pub eCode: u16, // Event Code (0-51, see enums)
    pub eVal1: u16,       // Event value 1 (User level, see curLvl)
    pub eVal2: u16,       // Event value 2 (Dependent on event code)
    pub eVal3: u16,       // Event value 3 (Dependent on event code)
    pub eVal4: u16,       // Event value 4 (Dependent on event code)
}

// Operation Counter Structure
#[derive(Serialize, Deserialize, Debug)]
pub struct OperationCounter {
    pub ckSt: u32,      // Cooks Started
    pub ckFin: u32,     // Cooks Finished
    pub fanCyc: u32,    // FAN On/Off cycles
    pub blCyc: u32,     // Blower Low Cycles
    pub blHCyc: u32,    // Blower High Cycles
    pub doorCl: u32,    // Door Closures
    pub htrHrs: u32,    // Heater runtime (0.01-hour increments)
    pub catHrs: u32,    // Catalyst runtime (0.01-hour increments)
    pub oCRC: u32,      // Checksum (32-bit CRC)
}


#[derive(Serialize, Deserialize, Debug)]
pub enum FryingState {
    Idle = 0,            // Fryer idle, no heating
    Heating = 1,         // Heating to setpoint
    ReadyToCook = 2,     // Ready to cook
    RunningRecipe = 3,   // Running a recipe
    RecipePaused = 4,    // Recipe paused
    RecipeContinued = 5, // Recipe continued
    RecipeComplete = 6,  // Recipe complete
}

// Enum for User Level
#[derive(Serialize, Deserialize, Debug)]
pub enum UserLevel {
    GeneralUser = 16,  // General user
    Supervisor = 17,   // Supervisor
    Technician = 18,   // Technician
    Factory = 19,      // Factory
}


// Enum for Event Codes (no associated values since they're stored in LogEvent)
#[repr(u16)]
#[derive(Debug, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub enum EventCode {
    ProbeFault = 3,          // Probe Fault
    ProbeCalibration = 7,    // Probe Calibration
    MemoryWriteError = 8,    // Memory Write Error
    MemoryReadError = 9,     // Memory Read Error
    FactoryReset = 12,       // Factory Reset
    ConfigurationChange = 14,// Configuration Change
    PasswordError = 15,      // Password Error
    RecipeChange = 18,       // Recipe Change
    RecipeGroupImport = 19,  // Recipe/Group Import
    RecipeGroupExport = 20,  // Recipe/Group Export
    ConfigurationImport = 21,// Configuration Import
    ConfigurationExport = 22,// Configuration Export
    GroupChange = 24,        // Group Change
    LogExport = 26,          // Log Export
    OverTempAlarm = 28,      // Over Temp Alarm
    USBWriteFileError = 31,  // USB Write File Error
    USBReadFileError = 32,   // USB Read File Error
    USBDataWriteError = 33,  // USB Data Write Error
    USBReadDataError = 34,   // USB Read Data Error
    FirmwareUpdate = 35,     // Firmware Update
    TempHWError = 37,        // Temperature Hardware Error
}



#[derive(Serialize, Deserialize, Debug)]
pub struct RealtimeStatusWrapper {
    pub RealtimeStatus: RealtimeStatus, // Ensures "RealtimeStatus" is the top-level key
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecipeWrapper {
    pub Recipe: Recipe, // Ensures "Recipe" is the top-level key
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GroupWrapper {
    pub Group: Group, // Ensures "Group" is the top-level key
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ControllerConfigWrapper {
    pub ControllerConfig: ControllerConfig, // Ensures "ControllerConfig" is the top-level key
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogEventWrapper {
    pub LogEvent: LogEvent, // Ensures "LogEvent" is the top-level key
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OperationCounterWrapper {
    pub OperationCounter: OperationCounter, // Ensures "OperationCounter" is the top-level key
}