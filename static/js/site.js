(() => {
  const initDeferredStyles = () => {
    for (const link of document.querySelectorAll("[data-deferred-stylesheet]")) {
      if (link.rel !== "stylesheet") {
        link.rel = "stylesheet";
      }
    }
  };

  const normalizePath = (value) => {
    const normalized = (value || "/").replace(/\/+$/, "");
    return normalized === "" ? "/" : normalized;
  };

  const currentPath = normalizePath(window.location.pathname);

  const initHeader = () => {
    const header = document.querySelector("[data-site-header]");
    if (!header) {
      return;
    }

    const body = document.body;
    const mobileToggle = header.querySelector("[data-mobile-toggle]");
    const mobilePanel = header.querySelector("[data-mobile-panel]");
    const servicesMenu = header.querySelector("[data-services-menu]");
    const servicesToggle = header.querySelector("[data-services-toggle]");
    const servicesPanel = header.querySelector("[data-services-panel]");
    const mobileServicesToggle = header.querySelector("[data-mobile-services-toggle]");
    const mobileServicesPanel = header.querySelector("[data-mobile-services-panel]");

    let mobileOpen = false;
    let servicesOpen = false;
    let mobileServicesOpen = false;

    const setScrolled = () => {
      header.classList.toggle("is-scrolled", window.scrollY > 24);
    };

    const setMobileOpen = (open) => {
      mobileOpen = open;
      if (mobilePanel) {
        mobilePanel.hidden = !open;
      }
      if (mobileToggle) {
        mobileToggle.classList.toggle("is-open", open);
        mobileToggle.setAttribute("aria-expanded", String(open));
      }
      body.classList.toggle("site-mobile-open", open && window.innerWidth < 768);
    };

    const setServicesOpen = (open) => {
      servicesOpen = open;
      if (servicesPanel) {
        servicesPanel.hidden = !open;
      }
      if (servicesToggle) {
        servicesToggle.setAttribute("aria-expanded", String(open));
      }
      const chevron = servicesToggle?.querySelector(".fa-chevron-down");
      chevron?.classList.toggle("rotate-180", open);
    };

    const setMobileServicesOpen = (open) => {
      mobileServicesOpen = open;
      if (mobileServicesPanel) {
        mobileServicesPanel.hidden = !open;
      }
      if (mobileServicesToggle) {
        mobileServicesToggle.setAttribute("aria-expanded", String(open));
      }
      const chevron = mobileServicesToggle?.querySelector(".fa-chevron-down");
      chevron?.classList.toggle("rotate-180", open);
    };

    const isActive = (pathValue) => {
      const paths = (pathValue || "")
        .split(",")
        .map((item) => normalizePath(item.trim()))
        .filter(Boolean);

      return paths.some((normalized) => {
        if (normalized === "/") {
          return currentPath === "/";
        }
        return currentPath === normalized || currentPath.startsWith(`${normalized}/`);
      });
    };

    for (const link of header.querySelectorAll("[data-nav-path]")) {
      const path = link.getAttribute("data-nav-path");
      const active = path ? isActive(path) : false;
      if (active) {
        link.classList.add(
          link.classList.contains("site-mobile-link")
            ? "site-mobile-link-active"
            : "site-nav-link-active",
        );
        link.setAttribute("aria-current", "page");
      }
    }

    const servicesGroupActive = isActive("/services");
    if (servicesGroupActive) {
      servicesToggle?.classList.add("site-nav-link-active");
      mobileServicesToggle?.classList.add("site-mobile-link-active");
    }

    mobileToggle?.addEventListener("click", () => {
      setMobileOpen(!mobileOpen);
    });

    servicesToggle?.addEventListener("click", (event) => {
      event.preventDefault();
      setServicesOpen(!servicesOpen);
    });

    mobileServicesToggle?.addEventListener("click", () => {
      setMobileServicesOpen(!mobileServicesOpen);
    });

    servicesMenu?.addEventListener("mouseenter", () => {
      setServicesOpen(true);
    });

    servicesMenu?.addEventListener("mouseleave", () => {
      setServicesOpen(false);
    });

    document.addEventListener("click", (event) => {
      if (servicesOpen && servicesMenu && !servicesMenu.contains(event.target)) {
        setServicesOpen(false);
      }
    });

    window.addEventListener(
      "resize",
      () => {
        if (window.innerWidth >= 1024) {
          setMobileOpen(false);
          setMobileServicesOpen(false);
        }
      },
      { passive: true },
    );

    window.addEventListener(
      "scroll",
      () => {
        setScrolled();
      },
      { passive: true },
    );

    window.addEventListener("keydown", (event) => {
      if (event.key === "Escape") {
        setServicesOpen(false);
        setMobileOpen(false);
        setMobileServicesOpen(false);
      }
    });

    setScrolled();
    setMobileOpen(false);
    setServicesOpen(false);
    setMobileServicesOpen(false);
  };

  const initRequestQuoteModal = () => {
    const modal = document.querySelector("[data-request-quote-modal]");
    if (!modal) {
      return;
    }

    const dialog = modal.querySelector("[data-request-quote-dialog]");
    const closeButton = modal.querySelector("[data-request-quote-close]");
    const successMessage = modal.querySelector("[data-request-quote-success]");
    const redirectInput = modal.querySelector("[data-request-quote-redirect]");
    const primaryField = modal.querySelector("[data-request-quote-primary]");
    let requestQuoteSuccess = false;
    let requestQuoteTrigger = null;

    const updateRedirect = () => {
      if (!redirectInput) {
        return;
      }
      const url = new URL(window.location.href);
      url.searchParams.delete("request_quote");
      const cleanPath = `${url.pathname}${url.search}${url.hash}` || "/";
      redirectInput.value = `${cleanPath}${cleanPath.includes("?") ? "&" : "?"}request_quote=success`;
    };

    const focusTarget = () => {
      const target = requestQuoteSuccess ? closeButton : primaryField;
      target?.focus({ preventScroll: true });
    };

    const openModal = (trigger = document.activeElement, success = false) => {
      updateRedirect();
      requestQuoteSuccess = success;
      requestQuoteTrigger = trigger instanceof HTMLElement ? trigger : null;
      modal.hidden = false;
      successMessage.hidden = !success;
      document.body.classList.add("site-modal-open");
      focusTarget();
    };

    const closeModal = () => {
      modal.hidden = true;
      document.body.classList.remove("site-modal-open");
      if (requestQuoteSuccess) {
        const url = new URL(window.location.href);
        url.searchParams.delete("request_quote");
        window.history.replaceState({}, "", `${url.pathname}${url.search}${url.hash}`);
      }
      requestQuoteSuccess = false;
      successMessage.hidden = true;
      requestQuoteTrigger?.focus({ preventScroll: true });
      requestQuoteTrigger = null;
    };

    document.addEventListener("open-request-quote", () => openModal());

    document.addEventListener("click", (event) => {
      const link = event.target.closest("a[href='/request-quote']");
      if (link && !event.defaultPrevented && event.button === 0) {
        if (!(event.metaKey || event.ctrlKey || event.shiftKey || event.altKey)) {
          event.preventDefault();
          openModal(link, false);
          return;
        }
      }

      if (event.target === modal) {
        closeModal();
      }
    });

    closeButton?.addEventListener("click", closeModal);

    window.addEventListener("keydown", (event) => {
      if (event.key === "Escape" && !modal.hidden) {
        closeModal();
      }
    });

    const url = new URL(window.location.href);
    if (url.searchParams.get("request_quote") === "success") {
      openModal(document.body, true);
    } else {
      updateRedirect();
    }

    if (dialog) {
      dialog.addEventListener("click", (event) => event.stopPropagation());
    }
  };

  const initNewsletterForms = () => {
    const forms = document.querySelectorAll("[data-newsletter-form]");

    for (const form of forms) {
      const message = form.querySelector("[data-newsletter-message]");
      if (!message) {
        continue;
      }

      form.addEventListener("submit", async (event) => {
        event.preventDefault();

        const formData = new FormData(form);
        const body = new URLSearchParams();

        for (const [key, value] of formData.entries()) {
          body.append(key, value.toString());
        }

        try {
          const response = await fetch("/newsletter/subscribe", {
            method: "POST",
            headers: {
              Accept: "application/json",
              "Content-Type": "application/x-www-form-urlencoded",
            },
            body,
          });

          const payload = await response.json();
          message.textContent = payload.message;
          message.hidden = false;
          message.classList.remove(
            "hidden",
            "border-emerald-300",
            "bg-emerald-50",
            "text-emerald-800",
            "border-rose-300",
            "bg-rose-50",
            "text-rose-800",
          );

          if (response.ok && payload.success) {
            message.classList.add("border-emerald-300", "bg-emerald-50", "text-emerald-800");
            form.reset();
          } else {
            message.classList.add("border-rose-300", "bg-rose-50", "text-rose-800");
          }
        } catch (_error) {
          message.hidden = false;
          message.textContent = "Subscription failed. Please try again.";
          message.classList.remove("hidden", "border-emerald-300", "bg-emerald-50", "text-emerald-800");
          message.classList.add("border-rose-300", "bg-rose-50", "text-rose-800");
        }
      });
    }
  };

  const initBackToTop = () => {
    const button = document.getElementById("backToTop");
    if (!button) {
      return;
    }

    const sync = () => {
      button.classList.toggle("hidden", window.scrollY <= 300);
    };

    window.addEventListener("scroll", sync, { passive: true });
    button.addEventListener("click", () => {
      window.scrollTo({ top: 0, behavior: "smooth" });
    });
    sync();
  };

  document.addEventListener("DOMContentLoaded", () => {
    initDeferredStyles();
    initHeader();
    initRequestQuoteModal();
    initNewsletterForms();
    initBackToTop();
  });
})();
