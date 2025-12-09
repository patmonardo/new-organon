/**
 * Design System Colors
 * Material Design inspired color palette
 */

export const colors = {
  // Primary brand colors
  primary: {
    main: "#6750A4",
    light: "#EADDFF",
    dark: "#21005D",
    contrast: "#FFFFFF",
  },

  // Secondary colors
  secondary: {
    main: "#625B71",
    light: "#E8DEF8",
    dark: "#1D192B",
    contrast: "#FFFFFF",
  },

  // State colors
  success: {
    main: "#388E3C",
    light: "#C8E6C9",
    dark: "#1B5E20",
    contrast: "#FFFFFF",
  },

  error: {
    main: "#D32F2F",
    light: "#FFCDD2",
    dark: "#B71C1C",
    contrast: "#FFFFFF",
  },

  warning: {
    main: "#FFA000",
    light: "#FFECB3",
    dark: "#FF6F00",
    contrast: "#000000",
  },

  info: {
    main: "#0288D1",
    light: "#B3E5FC",
    dark: "#01579B",
    contrast: "#FFFFFF",
  },

  // Surface colors
  surface: {
    main: "#FFFBFE",
    dim: "#F4EFF4",
    container: "#F3EDF7",
  },

  // Background
  background: "#FFFFFF",

  // Text colors
  text: {
    primary: "#212121",
    secondary: "#757575",
    disabled: "#BDBDBD",
  },

  // Border/outline
  outline: "#79747E",

  // Neutral palette
  gray: {
    50: "#F8F9FA",
    100: "#F1F3F5",
    200: "#E9ECEF",
    300: "#DEE2E6",
    400: "#CED4DA",
    500: "#ADB5BD",
    600: "#6C757D",
    700: "#495057",
    800: "#343A40",
    900: "#212529",
  },
} as const;

export type Colors = typeof colors;

