(() => {
  const normalizePath = (value) => {
    const normalized = (value || "/").replace(/\/+$/, "");
    return normalized === "" ? "/" : normalized;
  };

  const initSidebar = () => {
    const currentPath = normalizePath(window.location.pathname);
    const links = document.querySelectorAll("[data-nav-match]");
    const sidebar = document.getElementById("dashboard-sidebar");
    const backdrop = document.getElementById("dashboard-sidebar-backdrop");
    const openButton = document.getElementById("dashboard-sidebar-open");
    const closeButton = document.getElementById("dashboard-sidebar-close");

    const setSidebarState = (open) => {
      if (!sidebar || !backdrop) {
        return;
      }

      sidebar.classList.toggle("translate-x-0", open);
      sidebar.classList.toggle("translate-x-[-110%]", !open);
      backdrop.classList.toggle("opacity-100", open);
      backdrop.classList.toggle("pointer-events-auto", open);
      backdrop.classList.toggle("opacity-0", !open);
      backdrop.classList.toggle("pointer-events-none", !open);
      document.body.classList.toggle("overflow-hidden", open);
    };

    openButton?.addEventListener("click", () => setSidebarState(true));
    closeButton?.addEventListener("click", () => setSidebarState(false));
    backdrop?.addEventListener("click", () => setSidebarState(false));

    window.addEventListener("resize", () => {
      if (window.innerWidth >= 1024) {
        setSidebarState(false);
      }
    });

    window.addEventListener("keydown", (event) => {
      if (event.key === "Escape") {
        setSidebarState(false);
      }
    });

    for (const link of links) {
      const href = normalizePath(link.getAttribute("href") || "/");
      const mode = link.getAttribute("data-nav-match");

      const isActive =
        mode === "exact"
          ? currentPath === href
          : currentPath === href || currentPath.startsWith(`${href}/`);

      if (isActive) {
        link.classList.add("dashboard-nav-link-active");
        link.setAttribute("aria-current", "page");
      }

      link.addEventListener("click", () => {
        if (window.innerWidth < 1024) {
          setSidebarState(false);
        }
      });
    }
  };

  const initInsightsFilters = () => {
    const searchInput = document.getElementById("insights-search");
    const statusFilter = document.getElementById("insights-status-filter");
    const categoryFilter = document.getElementById("insights-category-filter");
    const rows = [...document.querySelectorAll("[data-insight-row]")];
    const emptyState = document.getElementById("insights-empty-state");
    const resultsCount = document.getElementById("insights-results-count");

    if (!searchInput || !statusFilter || !categoryFilter || !resultsCount || rows.length === 0) {
      return;
    }

    const applyFilters = () => {
      const search = searchInput.value.trim().toLowerCase();
      const status = statusFilter.value;
      const category = categoryFilter.value;
      let visibleCount = 0;

      for (const row of rows) {
        const matchesSearch = !search || row.dataset.search.includes(search);
        const matchesCategory = category === "all" || row.dataset.category === category;
        const matchesStatus =
          status === "all" ||
          row.dataset.status === status ||
          (status === "featured" && row.dataset.featured === "true");

        const visible = matchesSearch && matchesCategory && matchesStatus;
        row.classList.toggle("hidden", !visible);

        if (visible) {
          visibleCount += 1;
        }
      }

      resultsCount.textContent = `Showing ${visibleCount} of ${rows.length} entries`;
      emptyState?.classList.toggle("hidden", visibleCount !== 0);
    };

    searchInput.addEventListener("input", applyFilters);
    statusFilter.addEventListener("change", applyFilters);
    categoryFilter.addEventListener("change", applyFilters);
    applyFilters();
  };

  document.addEventListener("DOMContentLoaded", () => {
    initSidebar();
    initInsightsFilters();
  });
})();
