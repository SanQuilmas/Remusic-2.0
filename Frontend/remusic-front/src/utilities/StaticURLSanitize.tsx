const sanitizePath = (url: string): string => {
  try {
    const parsed = new URL(url);
    if (
      parsed.hostname === "backend" &&
      parsed.pathname.startsWith("/static/")
    ) {
      return parsed.pathname;
    }
    return url;
  } catch {
    return url;
  }
};

export { sanitizePath };
