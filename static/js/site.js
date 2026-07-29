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
    let servicesCloseTimer = null;

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

    const clearServicesCloseTimer = () => {
      if (servicesCloseTimer !== null) {
        window.clearTimeout(servicesCloseTimer);
        servicesCloseTimer = null;
      }
    };

    const scheduleServicesClose = () => {
      clearServicesCloseTimer();
      servicesCloseTimer = window.setTimeout(() => {
        setServicesOpen(false);
        servicesCloseTimer = null;
      }, 140);
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
      clearServicesCloseTimer();
      setServicesOpen(!servicesOpen);
    });

    mobileServicesToggle?.addEventListener("click", () => {
      setMobileServicesOpen(!mobileServicesOpen);
    });

    servicesMenu?.addEventListener("mouseenter", () => {
      clearServicesCloseTimer();
      setServicesOpen(true);
    });

    servicesMenu?.addEventListener("mouseleave", () => {
      scheduleServicesClose();
    });

    servicesMenu?.addEventListener("focusin", () => {
      clearServicesCloseTimer();
      setServicesOpen(true);
    });

    servicesMenu?.addEventListener("focusout", () => {
      window.setTimeout(() => {
        if (servicesMenu && !servicesMenu.contains(document.activeElement)) {
          scheduleServicesClose();
        }
      }, 0);
    });

    document.addEventListener("click", (event) => {
      if (servicesOpen && servicesMenu && !servicesMenu.contains(event.target)) {
        clearServicesCloseTimer();
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
        clearServicesCloseTimer();
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

  const initFooter = () => {
    const footer = document.querySelector(".site-footer");
    if (!footer) {
      return;
    }

    const reducedMotion = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
    const accordionGroups = Array.from(footer.querySelectorAll("[data-footer-accordion]"));
    const revealItems = Array.from(footer.querySelectorAll("[data-footer-reveal]"));

    const syncAccordion = () => {
      const mobile = window.innerWidth < 1024;

      for (const group of accordionGroups) {
        const toggles = Array.from(group.querySelectorAll("[data-footer-toggle]"));

        toggles.forEach((toggle, index) => {
          const panel = toggle.parentElement?.querySelector("[data-footer-panel]");
          if (!panel) {
            return;
          }

          if (!mobile) {
            toggle.disabled = true;
            toggle.setAttribute("aria-expanded", "true");
            panel.hidden = false;
            return;
          }

          toggle.disabled = false;
          const shouldOpen = index === 0;
          toggle.setAttribute("aria-expanded", String(shouldOpen));
          panel.hidden = !shouldOpen;
        });
      }
    };

    for (const group of accordionGroups) {
      const toggles = Array.from(group.querySelectorAll("[data-footer-toggle]"));

      for (const toggle of toggles) {
        toggle.addEventListener("click", () => {
          if (window.innerWidth >= 1024) {
            return;
          }

          const panel = toggle.parentElement?.querySelector("[data-footer-panel]");
          if (!panel) {
            return;
          }

          const nextOpen = toggle.getAttribute("aria-expanded") !== "true";

          for (const siblingToggle of toggles) {
            const siblingPanel = siblingToggle.parentElement?.querySelector("[data-footer-panel]");
            siblingToggle.setAttribute("aria-expanded", "false");
            if (siblingPanel) {
              siblingPanel.hidden = true;
            }
          }

          toggle.setAttribute("aria-expanded", String(nextOpen));
          panel.hidden = !nextOpen;
        });
      }
    }

    if (reducedMotion) {
      for (const item of revealItems) {
        item.classList.add("is-visible");
      }
    } else if ("IntersectionObserver" in window) {
      const observer = new IntersectionObserver(
        (entries) => {
          for (const entry of entries) {
            if (entry.isIntersecting) {
              entry.target.classList.add("is-visible");
              observer.unobserve(entry.target);
            }
          }
        },
        { threshold: 0.14, rootMargin: "0px 0px -40px 0px" },
      );

      for (const item of revealItems) {
        observer.observe(item);
      }
    } else {
      for (const item of revealItems) {
        item.classList.add("is-visible");
      }
    }

    window.addEventListener("resize", syncAccordion, { passive: true });
    syncAccordion();
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

  const initInsightsHub = () => {
    const hub = document.querySelector("[data-insights-hub]");
    if (!hub) {
      return;
    }

    document.documentElement.classList.add("insights-enhanced");

    const progressBar = document.querySelector("[data-insights-progress]");
    const cards = Array.from(hub.querySelectorAll("[data-insight-card]"));
    const filterButtons = Array.from(hub.querySelectorAll("[data-category-filter]"));
    const searchInputs = Array.from(hub.querySelectorAll("[data-insights-search-input]"));
    const resetButtons = Array.from(hub.querySelectorAll("[data-insights-reset]"));
    const emptyState = hub.querySelector("[data-insights-empty-state]");
    const resultsCount = hub.querySelector("[data-insights-results-count]");
    const resultsPanels = Array.from(hub.querySelectorAll("[data-insights-search-results]"));
    const reducedMotion = window.matchMedia("(prefers-reduced-motion: reduce)").matches;

    let activeCategory = "all";
    let query = "";
    let searchIndex = [];

    try {
      const payload = document.getElementById("insights-search-index");
      searchIndex = payload?.textContent ? JSON.parse(payload.textContent) : [];
    } catch (_error) {
      searchIndex = [];
    }

    const syncInputs = () => {
      for (const input of searchInputs) {
        if (input.value !== query) {
          input.value = query;
        }
      }
    };

    const updateFilterButtons = () => {
      for (const button of filterButtons) {
        const matches = (button.dataset.categoryFilter || "all") === activeCategory;
        button.classList.toggle("is-active", matches);
        if (matches) {
          button.setAttribute("aria-pressed", "true");
        } else {
          button.removeAttribute("aria-pressed");
        }
      }
    };

    const updateProgress = () => {
      if (!progressBar) {
        return;
      }

      const scrollable = document.documentElement.scrollHeight - window.innerHeight;
      const ratio = scrollable > 0 ? Math.min(window.scrollY / scrollable, 1) : 0;
      progressBar.style.transform = `scaleX(${ratio})`;
    };

    const renderSearchResults = () => {
      const normalizedQuery = query.trim().toLowerCase();
      const shouldShow = normalizedQuery.length >= 2;

      for (const panel of resultsPanels) {
        if (!shouldShow) {
          panel.hidden = true;
          panel.innerHTML = "";
          continue;
        }

        const matches = searchIndex
          .filter((item) => {
            const haystack = [
              item.title,
              item.excerpt,
              item.author,
              item.category,
              item.category_key,
            ]
              .join(" ")
              .toLowerCase();
            const categoryMatch =
              activeCategory === "all" || item.category_key === activeCategory;
            return categoryMatch && haystack.includes(normalizedQuery);
          })
          .slice(0, 6);

        panel.hidden = false;
        if (matches.length === 0) {
          panel.innerHTML = `
            <div class="insights-dark-soft rounded-2xl border px-4 py-4 text-sm text-slate-300">
              No archive results matched that search. Try a broader keyword or reset the filters.
            </div>
          `;
          continue;
        }

        panel.innerHTML = matches
          .map(
            (item) => `
              <a href="${item.public_url}" class="insights-dark-soft block rounded-2xl border px-4 py-4 transition hover:border-cyan-300 hover:bg-white/10">
                <div class="flex flex-wrap items-center gap-2 text-xs font-black uppercase tracking-[0.18em] text-cyan-300">
                  <span>${item.category}</span>
                  <span class="text-slate-500">•</span>
                  <span class="text-slate-300">${item.published_date_label}</span>
                  <span class="text-slate-500">•</span>
                  <span class="text-slate-300">${item.reading_time_label}</span>
                </div>
                <div class="mt-2 text-base font-black tracking-tight text-white">${item.title}</div>
                <p class="mt-2 text-sm leading-6 text-slate-300">${item.excerpt}</p>
              </a>
            `,
          )
          .join("");
      }
    };

    const applyFilters = () => {
      const normalizedQuery = query.trim().toLowerCase();
      let visibleCount = 0;

      for (const card of cards) {
        const category = card.dataset.category || "";
        const haystack = (card.dataset.search || "").toLowerCase();
        const categoryMatch = activeCategory === "all" || category === activeCategory;
        const queryMatch = !normalizedQuery || haystack.includes(normalizedQuery);
        const visible = categoryMatch && queryMatch;

        card.classList.toggle("insights-card-hidden", !visible);
        card.hidden = !visible;
        if (visible) {
          visibleCount += 1;
        }
      }

      if (resultsCount) {
        resultsCount.textContent = `${visibleCount} article${visibleCount === 1 ? "" : "s"}`;
        resultsCount.dataset.empty = visibleCount === 0 ? "true" : "false";
      }

      if (emptyState) {
        emptyState.hidden = visibleCount !== 0;
      }

      updateFilterButtons();
      renderSearchResults();
    };

    for (const input of searchInputs) {
      input.addEventListener("input", () => {
        query = input.value;
        syncInputs();
        applyFilters();
      });
    }

    for (const button of filterButtons) {
      button.addEventListener("click", (event) => {
        const targetCategory = button.dataset.categoryFilter || "all";

        if (button.tagName === "A") {
          event.preventDefault();
        }

        activeCategory = targetCategory;
        applyFilters();
      });
    }

    for (const button of resetButtons) {
      button.addEventListener("click", () => {
        query = "";
        activeCategory = "all";
        syncInputs();
        applyFilters();
      });
    }

    const revealItems = Array.from(hub.querySelectorAll("[data-reveal]"));
    if (reducedMotion) {
      for (const item of revealItems) {
        item.classList.add("is-visible");
      }
    } else if ("IntersectionObserver" in window) {
      const observer = new IntersectionObserver(
        (entries) => {
          for (const entry of entries) {
            if (entry.isIntersecting) {
              entry.target.classList.add("is-visible");
              observer.unobserve(entry.target);
            }
          }
        },
        { threshold: 0.12, rootMargin: "0px 0px -40px 0px" },
      );

      for (const item of revealItems) {
        observer.observe(item);
      }
    } else {
      for (const item of revealItems) {
        item.classList.add("is-visible");
      }
    }

    window.addEventListener("scroll", updateProgress, { passive: true });
    updateProgress();
    syncInputs();
    applyFilters();
  };

  document.addEventListener("DOMContentLoaded", () => {
    initDeferredStyles();
    initHeader();
    initRequestQuoteModal();
    initNewsletterForms();
    initFooter();
    initBackToTop();
    initInsightsHub();
  });
})();
